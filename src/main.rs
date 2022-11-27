use regex::Regex;

fn main() {
    println!("Hello, world!");

    //Expresiones regulares
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    //(\d+) \s? \+ \s? (\d+) 

    //Traemos datos del usuario
    println!("Por favor ingrese su expresión");
    let mut expresion = String::new();
    std::io::stdin().read_line(&mut expresion).unwrap();

    //Multiplicación
    loop {
        //Aplicar operaciones
        let caps = re_mul.captures(expresion.as_str());

        if caps.is_none(){
            break;
        }

        let caps = caps.unwrap();

        let caps_expression= caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        println!("{:?}",caps);

        let addition = left_value * right_value;
        expresion = expresion.replace(caps_expression, &addition.to_string())

    }

    //Suma
    loop {
        //Aplicar operaciones
        let caps = re_add.captures(expresion.as_str());

        if caps.is_none(){
            break;
        }

        let caps = caps.unwrap();

        let caps_expression= caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        println!("{:?}",caps);

        let addition = left_value + right_value;
        expresion = expresion.replace(caps_expression, &addition.to_string())

    }

    
    //Mostrar resultados
    println!("Resultado: {}", expresion)
}
