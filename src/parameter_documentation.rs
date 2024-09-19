use std::vec::IntoIter;

#[derive(Clone)]
pub struct ParameterDocumentation {
    names: Option<Vec<&'static str>>,
    descriptions: Option<Vec<&'static str>>,
    expected_types: Option<Vec<&'static str>>,
}

impl IntoIterator for ParameterDocumentation {
    type Item = (&'static str, &'static str, &'static str);

    type IntoIter = IntoIter<(&'static str, &'static str, &'static str)>;

    fn into_iter(self) -> Self::IntoIter {
        if self.names.is_none() {
            return Vec::<Self::Item>::new().into_iter();
        }

        let names = self.names.unwrap();
        let descs = self.descriptions.unwrap();
        let types = self.expected_types.unwrap();

        let mut out = Vec::<Self::Item>::with_capacity(names.len());

        for i in 0..names.len() {
            out.push((names[i], descs[i], types[i]));
        }        
        
        out.into_iter()
    }
}

impl ParameterDocumentation {
    pub fn new(
        names: Vec<&'static str>,
        descriptions: Vec<&'static str>,
        expected_types: Vec<&'static str>,
    ) -> Self {
        if names.len() != descriptions.len() || descriptions.len() != expected_types.len() {
            panic!("All Vecs passed need to be the same length");
        }

        Self {
            names: Some(names),
            descriptions: Some(descriptions),
            expected_types: Some(expected_types),
        }
    }
}
