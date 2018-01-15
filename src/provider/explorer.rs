use super::{SharedProviders, ProviderFolder, NavigationError};

pub struct Explorer {
    pub path: Vec<String>,
    providers: SharedProviders
}

impl Explorer {
    pub fn new(providers: SharedProviders) -> Explorer {
        Explorer {
            path: vec![],
            providers
        }
    }

    pub fn navigate_absolute(&mut self, path: String) {
        let mut absolute = vec![];
        let mut current = path.as_str();
        while !current.is_empty() {
            let layer = match current.find('/') {
                Some(index) => {
                    let layer = &current[..index];
                    current = &current[index + 1..];
                    layer
                },
                None => {
                    let copy = current.clone();
                    current = "";
                    copy
                }
            };
            absolute.push(layer.to_owned());
        }
        self.path = absolute;
    }

    pub fn navigate(&mut self, path: String) {
        self.path.push(path);
    }

    pub fn go_up(&mut self) {
        self.path.pop();
    }

    pub fn path(&self) -> String {
        self.path
            .iter()
            .fold(String::new(), |mut a, b| {
                a.push_str(format!("{}/", b).as_str());
                a
            })
    }

    fn get_root(&self) -> ProviderFolder {
        let folders = self.providers
            .iter()
            .map(|provider| provider.read().unwrap().title().to_owned())
            .collect();
        ProviderFolder {
            folders,
            items: vec![]
        }
    }

    pub fn items(&self) -> Result<ProviderFolder, NavigationError> {
        let root = self.get_root();
        match self.path.len() {
            0 => Ok(root),
            1 => {
                let path = &self.path[0];
                let provider = self.providers
                    .iter()
                    .find(|provider| provider.read().unwrap().title() == path);
                provider
                    .ok_or(NavigationError::PathNotFound)
                    .map(|provider| provider.read().unwrap().root())
            },
            _ => {
                let path = &self.path[0];
                let provider = self.providers
                    .iter()
                    .find(|provider| provider.read().unwrap().title() == path);
                let path = &self.path[1..];
                provider
                    .ok_or(NavigationError::PathNotFound)
                    .and_then(|provider| provider.read().unwrap().navigate(path.to_vec()))
            }
        }
    }
}