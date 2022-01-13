#[cfg(test)]
mod interface_test {

    #[test]
    fn private_interface() {
        let ts_declaration = r#"
            export interface HotPot {
                price : number;
                name : string;
                chilly : boolean;
                size : string[];
            }
            "#;
    }
}
