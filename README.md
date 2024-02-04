# pdp-food-rest-api

##### Table of Contents  
- [Introducción](#introducción)  
- [¿Cómo utilizar?](#cómo-utilizar)
  - [Instalación](#instalación)
    - [Rustc](#rustc)
    - [Postgres](#postgres)
    - [Diesel](#diesel)
  - [Ejecución](#ejecución)
- [¿Cómo testear?](#cómo-testear)


## Introducción
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

### Instalación
Es necesario tener instalado las siguientes herramientas:

#### Rustc
Podemos seguir los pasos descritos en la [documentación](https://www.rust-lang.org/tools/install). 
> [!NOTE]
> **Es importante instalar la version _nightly_ para que sea compatible con el framework Rocket**. Para ello, realizar la instalación personalizada (2) y en el segundo paso, elegir la versión _nightly_ (por defecto está seleccionada la version _stable_). En las demás opciones, no modificar nada. Luego proceder con la instalación normalmente.

#### Postgres
Recomendamos utilizar Docker. Siguiendo la documentación oficial, podemos instalar [Docker compose](https://docs.docker.com/compose/install/linux/). Agregamos las siguientes variables al archivo `.env`

```bash
HOST=127.0.0.1
USER=root
PASSWORD=root
DATABASE=food

ADMIN_EMAIL="pgadmin@gmail.com"
ADMIN_PASSWORD="admin"
```

Luego, en la carpeta raíz donde se encuentra _docker-compose.yml_, ejecutamos en _detach mode_

```bash
sudo docker-compose run -d 
```
> [!WARNING]
> Utilizar Docker es solo una sugerencia. Se puede utilizar postgres instalado en el sistema, creando adecuadamente la base de datos con el mismo nombre definido en el `.env`.

Podemos previamente modificar el archivo `docker-compose.yml` con los puertos y variables a gusto. Una vez listo, se habrán creado dos contenedores.
Uno con la base de datos postgres `food` y otro para monitorear con `pgAdmin`. Podemos ingresar desde el navegador a `localhost:80` e ingresar el usuario y contraseña definidor en el `.env`. Allí creamos un server y nos conectamos a la base de datos que hemos creado. Es importante que el host sea **postgres**. 
Hasta este punto, la base de datos está creada pero sin tablas. 

#### Diesel
Para instalar Diesel CLI, podemos seguir los pasos de la [documentacion](https://diesel.rs/guides/getting-started).

> [!WARNING]
> Si surge el error `error: linking with 'cc' failed: exit status: 1`, una solución podría ser ejecutar el siguiente comando 
> ```bash
> sudo apt install libpq-dev
> ```

Cuando esté todo bien, tenemos que crear/cargar la migración. En este caso, en la carpeta _migrations_ ya hemos definido los scripts para iniciar y terminar con la base de datos. Por lo tanto, en una terminal ejecutamos el siguiente comando y se crearán las tablas.


```bash
diesel migration run
```

> [!WARNING]
> Quizás se deba crear el rol 'postgres', o podemos eliminar la asignación de owner en cada tabla dentro del script.

Con pgAdmin desde el navegador podremos corroborar que las tablas se hayan creado tal cuál se ha definido en el script `up.sql`.

### Ejecución
Una vez instaladas las herramientas mencionadas, y con los contenedores corriendo (al menos postgres), iniciamos la API con el siguiente comando

```bash
cargo run
```

La API estará disponible en el puerto 8000 (o en el que se haya definido). 

## ¿Cómo testear?
Hemos exportado en formato Json la colección de Postman que utilizamos en el proceso de desarrollo. Por lo tanto, podemos importar la misma en la aplicación (File -> Import). Descargar [aquí](https://github.com/ginos1998/pdp-food-rest-api/blob/develop/test-api/postman/pdp-food-rest-api.postman_collection.json).

La misma contiene todos los métodos HTTP para cada entity.
