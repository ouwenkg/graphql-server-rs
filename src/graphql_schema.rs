use crate::schema::members;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use juniper::RootNode;
use std::env;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[derive(Queryable)]
pub struct Member {
    id: i32,
    name: String,
    knockouts: i32,
    team_id: i32,
}

#[juniper::object(description = "A member of a team")]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn knockouts(&self) -> i32 {
        self.knockouts
    }

    pub fn team_id(&self) -> i32 {
        self.team_id
    }
}

#[derive(Queryable)]
pub struct Team {
    id: i32,
    name: String,
}

#[juniper::object(description = "A team of members")]
impl Team {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn members(&self) -> Vec<Member> {
        use crate::schema::members::dsl::*;

        let connection = establish_connection();
        members
            .filter(team_id.eq(self.id))
            .limit(100)
            .load::<Member>(&connection)
            .expect("Error loading members")
    }
}

pub struct QueryRoot {}

#[juniper::object]
impl QueryRoot {
    pub fn members() -> Vec<Member> {
        // vec![
        //     Member {
        //         id: 1,
        //         name: "m1".to_owned(),
        //     },
        //     Member {
        //         id: 2,
        //         name: "m2".to_owned(),
        //     },
        // ]
        use crate::schema::members::dsl::*;

        let connection = establish_connection();
        members
            .limit(100)
            .load::<Member>(&connection)
            .expect("Error load members")
    }

    pub fn teams() -> Vec<Team> {
        use crate::schema::teams::dsl::*;

        let connection = establish_connection();
        teams
            .limit(10)
            .load::<Team>(&connection)
            .expect("Error load teams")
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn create_member(data: NewMember) -> Member {
        let connection = establish_connection();
        diesel::insert_into(crate::schema::members::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving new post")
    }
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "members"]
pub struct NewMember {
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
