use super::{error::Error, response::Response};
use path_tree::PathTree;
#[derive(Clone)]
pub struct Router {
    pub root: PathTree<Response>,
}

impl Router {
    pub async fn new(path: &'static str, res: Response) -> Result<Router, Error> {
        let mut root: PathTree<Response> = PathTree::new();
        let _ = root.insert(path, res);

        Ok(Router { root })
    }

    pub async fn insert(&mut self, path: &'static str, res: Response) {
        let _ = self.root.insert(path, res);
    }
}
