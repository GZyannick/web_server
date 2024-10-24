use std::collections::HashMap;

use super::{
    error::Error,
    response::{status_code::HttpStatusCode, Response},
};
use path_tree::PathTree;
#[derive(Clone)]
pub struct Router {
    pub root: PathTree<Response>,
}

impl Router {
    pub async fn new(
        path: &'static str,
        status_code: HttpStatusCode,
        headers: Option<HashMap<String, String>>,
        content: &'static str,
    ) -> Result<Router, Error> {
        let mut root: PathTree<Response> = PathTree::new();
        let res = Response::new(status_code, headers, content).await?;
        let _ = root.insert(path, res);

        Ok(Router { root })
    }

    pub async fn insert(
        &mut self,
        path: &'static str,
        status_code: HttpStatusCode,
        headers: Option<HashMap<String, String>>,
        content: &'static str,
    ) -> Result<(), Error> {
        let res = Response::new(status_code, headers, content).await?;
        let _ = self.root.insert(path, res);
        Ok(())
    }
}
