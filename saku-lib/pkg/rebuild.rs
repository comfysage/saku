use crate::prelude::*;

use crate::util::constants;
use crate::util::io;
use crate::util::msg;
use crate::util::{filepath, path};

use super::pkg::Pkg;

#[derive(Debug)]
struct Node {
    /// store dir
    store_dir: String,
    /// as '/share/path/to/file'
    pub path: String,
    children: Vec<Node>,
}

impl Node {
    pub fn create_root(path: &str) -> Self {
        Node {
            store_dir: path.to_string(),
            path: String::from(""),
            children: vec![],
        }
    }
    pub fn new(store_dir: &str, path: &str) -> Self {
        Self {
            store_dir: store_dir.to_string(),
            children: vec![],
            path: path.to_string(),
        }
    }
    fn from(store_dir: &str, path: &str) -> Result<Self> {
        let rel = filepath::get_relative(store_dir, path)?;
        Ok(Self::new(store_dir, &rel))
    }
    fn create(&self, path: &str) -> Result<Self> {
        let mut child = Node::from(&self.store_dir, &path)?;
        child.build()?;
        Ok(child)
    }
    fn abs(&self) -> String {
        filepath::join(&self.store_dir, &self.path)
    }
    pub fn build(&mut self) -> Result<()> {
        if self.is_bud() {
            return Ok(());
        }
        for entry in std::fs::read_dir(&self.abs())? {
            let entry = entry?;
            let path_bind = entry.path();
            let path = path_bind.to_str().ok_or(make_err!())?;
            let child = self.create(path)?;
            self.children.push(child);
        }
        Ok(())
    }
    fn is_bud(&self) -> bool {
        let is_dir = filepath::is_dir(&self.abs());
        !is_dir
    }
    /// link node that is not a bud
    fn link_as_parent(&self, root: &Node) -> Result<()> {
        let rel = &self.path;
        let root_path = filepath::join(&root.path, &rel);
        if !filepath::exists(&root_path) {
            self.link_bud(&root_path)?;
            return Ok(());
        }
        debug!("({root_path}) root target already exists; skipping");
        for child in self.children.iter() {
            child.link(root)?;
        }
        Ok(())
    }
    /// link node to root
    pub fn link(&self, root: &Node) -> Result<()> {
        trace!("({}) current node", &self.path);
        if !self.is_bud() {
            self.link_as_parent(root)?;
            return Ok(());
        }
        let rel = &self.path;
        let root_path = filepath::join(&root.path, &rel);
        if filepath::exists(&root_path) {
            debug!("({root_path}) root file already exists; cleaning up");
            std::fs::remove_file(&root_path)?;
        }
        self.link_bud(&root_path)?;
        Ok(())
    }
    fn link_bud(&self, path: &str) -> Result<()> {
        if filepath::exists(&path) {
            return Err(make_err!(IO, "file already exists"));
        }
        io::mkdir(filepath::parent_dir(&path)?)?;
        msg::link(&self.abs(), &path);
        io::link(&self.abs(), &path)?;
        Ok(())
    }

    fn has_linked(&self, root: &Node) -> Result<bool> {
        let rel = &self.path;
        let root_path = filepath::join(&root.path, &rel);
        if filepath::exists(&root_path) {
            let metadata = std::fs::symlink_metadata(&root_path)?;
            let filetype = metadata.file_type();
            let is_symlink = filetype.is_symlink();
            return Ok(is_symlink);
        }
        Ok(false)
    }
    pub fn unlink(&self, root: &Node) -> Result<()> {
        trace!("({}) current node", &self.path);
        if self.has_linked(root)? {
            let rel = &self.path;
            let root_path = filepath::join(&root.path, &rel);
            msg::remove_file(&root_path);
            std::fs::remove_file(&root_path)?;
            return Ok(());
        }
        if !self.is_bud() {
            for child in self.children.iter() {
                child.unlink(root)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Root(Node, Node);

impl Root {
    pub fn new(pkg: &Pkg) -> Result<Self> {
        let store_path = path::store_dir(&pkg.name);
        let root_path = &*constants::ROOT_DIR;
        if !filepath::exists(&store_path) {
            return Err(make_err!(IO, "store dir does not exist {store_path}"));
        }
        Ok(Self(
            Node::create_root(&store_path),
            Node::create_root(&root_path),
        ))
    }
    pub fn build(&mut self) -> Result<&Self> {
        self.0.build()?;
        self.1.build()?;
        Ok(self)
    }
    pub fn link(&self) -> Result<()> {
        self.0.link(&self.1)?;
        Ok(())
    }
    pub fn uninstall(&self) -> Result<()> {
        self.0.unlink(&self.1)?;
        Ok(())
    }
}
