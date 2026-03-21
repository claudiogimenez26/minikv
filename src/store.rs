use std::collections::HashMap;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, clave: String, valor: String) {
        self.data.insert(clave, valor);
    }

    pub fn get(&self, clave: &str) -> Option<&String> {
        self.data.get(clave)
    }

    pub fn delete(&mut self, clave: &str) {
        self.data.remove(clave);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }
}

 #[cfg(test)]
    mod tests {

        //use super::*;
        use std::collections::HashMap;

        #[test]
        fn test_insert_y_get() {

            let mut store = HashMap::new();

            store.insert("nombre".to_string(), "claudio".to_string());

            let valor = store.get("nombre").unwrap();

            assert_eq!(valor, "claudio");
        }

        #[test]
        fn test_delete_clave() {

            let mut store = HashMap::new();

            store.insert("edad".to_string(), "30".to_string());

            store.remove("edad");

            let valor = store.get("edad");

            assert!(valor.is_none());
        }

        #[test]
        fn test_length() {

            let mut store = HashMap::new();

            store.insert("a".to_string(), "1".to_string());
            store.insert("b".to_string(), "2".to_string());

            assert_eq!(store.len(), 2);
        }

    }