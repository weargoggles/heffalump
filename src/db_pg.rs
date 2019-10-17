use actix::fut;
use actix::{Actor, Addr, Context};
use actix::prelude::*;
use tokio::prelude::*;
use tokio_postgres::{connect, Client, NoTls, Statement};
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;


pub struct Database {
    state: State
}

enum State {
    Connected(Client),
    NotConnected,
}

impl Actor for Database {
    type Context = Context<Self>;
}

#[derive(Debug)]
pub enum DBError {
    EnvError(std::env::VarError),
    TlsError(native_tls::Error),
}

impl From<std::env::VarError> for DBError {
    fn from(e: std::env::VarError) -> DBError {
        DBError::EnvError(e)
    }
}

impl From<native_tls::Error> for DBError {
    fn from(e: native_tls::Error) -> DBError {
        DBError::TlsError(e)
    }
}

impl Database {

    pub fn connect(config: &str) -> Result<Addr<Database>, DBError>
    {
        let connector = {
            let pem = std::env::var("DATABASE_CA_CERT")?;
            let cert = Certificate::from_pem(&pem.into_bytes())?;
            let connector = TlsConnector::builder()
                .add_root_certificate(cert)
                .build()?;
            MakeTlsConnector::new(connector)
        };
        let connect = tokio_postgres::connect(&config, connector);
        let db = Database{ state: State::NotConnected }; 

        Ok(Database::create(|ctx| {
            
            let f = connect
                .map_err(|_| panic!("can't connect to postgres"))
                .into_actor(&db)
                .and_then(|(cl, conn), act, ctx| {
                    act.state = State::Connected(cl);
                    Arbiter::spawn(conn.map_err(|e| panic!("halp conn {}", e)));
                    fut::ok(())
                });
            ctx.wait(f);
            db
        }))
    }
}