# pdp-food-rest-api

## Introduccion
El repositorio contiene una API Rest hecha con Rust y el framework Rocket, para controlar un CRUD con planes de comida, recetas, menús diarios, entre otros. Los datos se leen y almacenan en una base de datos Postgres con el framework Diesel. 

## ¿Cómo utilizar?
Para ambas opciones:
- Clonar el repositorio y abrir una nueva terminal en la carpeta **pdp-food-rest-api**.
- Crear un archivo **.env** y agregar las siguientes variables de entorno
```bash
DATABASE_URL = postgres://username:password@localhost:5432/food
RUST_BACKTRACE=1
RUST_LOG=debug
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=8000
```

## Docker
Teniendo instalado [Docker compose](https://docs.docker.com/compose/install/linux/), ejecutamos docker en _detach mode_ y listo.

```bash
sudo docker-compose run -d 
```

## Manualmente
Es necesario tener instalado las siguientes herramientas:
- Rustc, siguiendo los pasos descritos en la [documentación](https://www.rust-lang.org/tools/install). **Es importante instalar la version _nightly_ para que sea compatible con el framework Rocket.**
- Diesel, podemos seguir los pasos de la [documentacion](https://diesel.rs/guides/getting-started).
- Crear una base de datos postgres

Una vez listo, ejecutamos el siguiente comando para iniciar la API
```bash
cargo run
```

## ¿Cómo testear?
Hemos exportado en formato Json la colección de Postman que utilizamos en el proceso de desarrollo. Por lo tanto, podemos importar la misma en la aplicación (File -> Import). Descargar [aquí](https://github.com/ginos1998/pdp-food-rest-api/blob/develop/test-api/postman/pdp-food-rest-api.postman_collection.json).
