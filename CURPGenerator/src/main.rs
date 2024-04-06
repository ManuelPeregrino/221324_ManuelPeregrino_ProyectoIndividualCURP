use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use regex::Regex;
use rand::{distributions::Uniform, Rng};


// TODO: Implement more comprehensive error handling.
// TODO: Add logging middleware for better monitoring.

// Data structure for input data.
#[derive(Deserialize)]
struct UserData {
    first_name: String,
    father_surname: String,
    mother_surname: String,
    birth_date: String, // Format: YYYY-MM-DD
    gender: char,       // 'H' for male, 'M' for female
    birth_state: String,
}

// Data structure for the response which includes the CURP.
#[derive(Serialize)]
struct CurpResponse {
    curp: String,
}

// The handler function to generate the CURP.
async fn generate_curp(user_data: web::Json<UserData>) -> impl Responder {
    //* Critical for performance: Efficient CURP generation logic.
    let curp = generate_curp_logic(&user_data).unwrap_or_else(|_| String::from("Error generating CURP"));

    //<> Regular update: We return the CURP wrapped in a JSON response.
    HttpResponse::Ok().json(CurpResponse { curp })
}

// Function to generate CURP logic.
fn generate_curp_logic(user_data: &UserData) -> Result<String, &'static str> {
    let mut curp = String::with_capacity(18);

    // Agregar la primera letra y la primera vocal interna del primer apellido
    let re = Regex::new(r"^[^AEIOU]*([AEIOU])").unwrap(); // Encuentra la primera vocal
    let first_surname_initial = user_data.father_surname.chars().next().unwrap_or_default();
    let first_vowel = re.captures(&user_data.father_surname.to_uppercase())
        .and_then(|caps| caps.get(1))
        .map_or_else(|| 'X', |m| m.as_str().chars().next().unwrap_or_default());

    curp.push(first_surname_initial);
    curp.push(first_vowel);

    // Agregar la primera letra del segundo apellido o 'X' si no está presente
    let second_surname_initial = user_data.mother_surname.chars().next().unwrap_or('X');
    curp.push(second_surname_initial);

    // Agregar la primera letra del nombre
    let name_initial = user_data.first_name.chars().next().unwrap_or_default();
    curp.push(name_initial);

    // Formato de la fecha de nacimiento YYMMDD
    let birth_date = &user_data.birth_date;
    if birth_date.len() == 10 {
        curp.push_str(&birth_date[2..4]); // Año
        curp.push_str(&birth_date[5..7]); // Mes
        curp.push_str(&birth_date[8..]);  // Día
    } else {
        return Err("Invalid birth date format");
    }

    // Agregar la letra correspondiente al género
    curp.push(user_data.gender);

    // Agregar las dos letras correspondientes a la entidad de nacimiento
    // Esta es una simplificación, debería haber una correspondencia completa entre entidades y códigos
    let birth_state_code = match user_data.birth_state.to_uppercase().as_str() {
        "DISTRITO FEDERAL" => "DF",
        "AGUASCALIENTES" => "AS",
        "BAJA CALIFORNIA" => "BC",
        "BAJA CALIFORNIA SUR" => "BS",
        "CAMPECHE" => "CC",
        "CHIAPAS" => "CS",
        "CHIHUAHUA" => "CH",
        "COAHUILA" => "CA",
        "COLIMA" => "CM",
        "DURANGO" => "DO",
        "ESTADO DE MEXICO" => "EM",
        "GUANAJUATO" => "GO",
        "GUERRERO" => "GR",
        "HIDALGO" => "HO",
        "JALISCO" => "JO",
        "MICHOACAN" => "MC",
        "MORELOS" => "MS",
        "NAYARIT" => "NT",
        "NUEVO LEON" => "NL",
        "OAXACA" => "OX",
        "PUEBLA" => "PA",
        "QUERETARO" => "QO",
        "QUINTANA ROO" => "QR",
        "SAN LUIS POTOSI" => "SL",
        "SINALOA" => "SA",
        "SONORA" => "SO",
        "TABASCO" => "TB",
        "TAMAULIPAS" => "TM",
        "TLAXCALA" => "TX",
        "VERACRUZ" => "VZ",
        "YUCATAN" => "YC",
        "ZACATECAS" => "ZS",
        // ... agregar otros estados aquí
        _ => "NE" // No Especificado o valor por defecto
    };
    curp.push_str(birth_state_code);

    // Falta agregar la lógica para las siguientes partes:
    // - Primeras consonantes internas de los apellidos y del nombre
    // - Dígitos para evitar duplicados
    // - Dígito verificador

    // Ejemplo de retorno de la CURP incompleta
    // Agregar la primera consonante interna de cada apellido y del nombre, si existen.
    let first_internal_consonant = |s: &str| -> char {
        s.chars()
            .skip(1) // Skip the first letter as we are looking for the first internal consonant.
            .find(|&c| matches!(c, 'B' | 'C' | 'D' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'M' | 'N' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'V' | 'W' | 'X' | 'Y' | 'Z'))
            .unwrap_or('X') // Use 'X' if there is no internal consonant.
    };

    // Add the first internal consonant of each part of the name to the CURP.
    curp.push(first_internal_consonant(&user_data.father_surname.to_uppercase()));
    curp.push(first_internal_consonant(&user_data.mother_surname.to_uppercase()));
    curp.push(first_internal_consonant(&user_data.first_name.to_uppercase()));

    // Generate a random uppercase alphanumeric character.
    let mut rng = rand::thread_rng(); // Mark 'rng' as mutable.
    let upper_case_range = Uniform::new_inclusive(b'A', b'Z'); // Range of uppercase letters.
    let random_uppercase_char = rng.sample(upper_case_range) as char;
    curp.push(random_uppercase_char);

    // Generate a random number between 0 and 9.
    let random_number = rng.gen_range(0..10).to_string().chars().next().unwrap();
    curp.push(random_number);

    Ok(curp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173") // El origen de tu frontend
            .allowed_methods(vec!["GET", "POST"]) // Métodos HTTP permitidos
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE]) // Encabezados permitidos
            .supports_credentials() // Si necesitas enviar cookies o encabezados de autorización
            .max_age(3600); // Tiempo máximo para el cache de la respuesta a la verificación previa de CORS

        App::new()
            .wrap(cors)
            .route("/generate_curp", web::post().to(generate_curp))
            .default_service(web::route().to(HttpResponse::NotFound))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}