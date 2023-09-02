pub struct Book {
    pub _id: String,
    pub name: String,
}

pub struct Movie {
    pub _id: String,
    pub name: String,
    
    pub budgetInMillions: u32,
    pub boxOfficeRevenueInMillions: u32,
    pub academyAwardNominations: u32,
    pub academyAwardWins: u32,
    pub rottenTomatesScore: u32,
}
