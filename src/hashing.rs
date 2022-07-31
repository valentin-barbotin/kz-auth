use argon2::{
    Argon2,
    Algorithm,
    Version,
    Params,
    password_hash::{
        Error,
        SaltString,
        rand_core::OsRng
    }, 
    PasswordVerifier,
    PasswordHasher,
    PasswordHash
};


macro_rules! get_argon {
    () => {
        // m_cost, t_cost, p_cost, output_len
        // memory size, number of iterations, parallelism, output length
        Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(2048, 4, 1, None).unwrap())
    };
}


pub fn verify_password(password: &[u8], hashed_password: &str) -> Result<bool, Error> {
    let hash = PasswordHash::new(hashed_password).unwrap();
    let argon2 = get_argon!();
    argon2.verify_password(password, &hash)?;
    Ok(true)
}

pub fn generate_hash(data: &str) -> String {
    let argon2 = get_argon!();
    let data = data.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2.hash_password(data, &salt).unwrap();
    hash.to_string()
}

// #[allow(unused_variables)]
// #[test]
// fn test() {
//     let hashed_password = "$argon2id$v=19$m=4096,t=3,p=1$GmTdOEYerIYwfxyUYoJnZg$gclnloEAGOYkV/BHjhcFUb3Q8XfhirTT2RQanLAepzA";
//     let password = b"ratio";
//     let hash = PasswordHash::new(hashed_password).unwrap();
//     let argon2 = get_argon!();
//     let check = argon2.verify_password(password, &hash).unwrap();
// }

// #[test]
// pub fn generate_password() {
//     let argon2 = get_argon!();
//     let password = b"ratio";
//     let salt = SaltString::generate(&mut OsRng);
//     let hash = argon2.hash_password(password, &salt).unwrap();
//     println!("hash = {:?}", hash.to_string());
// }