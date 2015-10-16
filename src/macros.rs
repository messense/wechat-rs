macro_rules! make_encodable(
    ($name:ident) => (
        impl ::rustc_serialize::Encodable for $name {
            fn encode<S: ::rustc_serialize::Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
                let json_obj = self.to_json();
                json_obj.encode(s)
            }
        }
    )
);
