pub fn test_ast() {
    let code = r#"
        <template>
            <div>
                <h1>Hello, world!</h1>
                <p>This is an example of a SFC component.</p>
            </div>
        </template>
    "#;

    let ast = get_ast(code);

    println!("{}", ast);
}
