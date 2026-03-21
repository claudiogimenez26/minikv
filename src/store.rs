use std::collections::HashMap;

///Estructura principal del sistema key-value Store.
///
///Almacena los pares clave-valor en memoria utilizando un HashMap.
pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    ///Crea un nuevo store vacío.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    ///Inserta o actualiza una clave con su valor.
    pub fn set(&mut self, clave: String, valor: String) {
        self.data.insert(clave, valor);
    }
    
    ///Obtiene el valor asociado a una clave, si existe.
    ///
    ///Devuelve `Some(valor)` si la clave existe o `None` si no existe.
    pub fn get(&self, clave: &str) -> Option<&String> {
        self.data.get(clave)
    }

    ///Elimina una clave del store si existe.
    pub fn delete(&mut self, clave: &str) {
        self.data.remove(clave);
    }

    ///Devuelve la cantidad de pares clave-valor almacenados en el store.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    ///Iterador sobre los pares clave-valor almacenados en el store.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_y_get() {
        let mut store = Store::new();

        store.set("nombre".to_string(), "claudio".to_string());

        let valor = store.get("nombre").unwrap();

        assert_eq!(valor, "claudio");
    }

    #[test]
    fn test_delete_clave() {
        let mut store = Store::new();

        store.set("edad".to_string(), "30".to_string());

        store.delete("edad");

        let valor = store.get("edad");

        assert!(valor.is_none());
    }

    #[test]
    fn test_length() {
        let mut store = Store::new();

        store.set("a".to_string(), "1".to_string());
        store.set("b".to_string(), "2".to_string());

        assert_eq!(store.len(), 2);
    }
}