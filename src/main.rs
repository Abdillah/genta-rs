use tracker_sparql;

fn main() {
    let query = "SELECT ?title nie:url(?u) WHERE {
        { ?u a nfo:FileDataObject }
        { ?u <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.semanticdesktop.org/ontologies/2007/03/22/nfo#Software> }
        { ?u <http://www.semanticdesktop.org/ontologies/2007/01/19/nie#title> ?title }
        FILTER( STRSTARTS(?u, 'file:///run/current-system/sw/share/applications/') || STRSTARTS(?u, 'file:///home/user/.local/share/applications/') )
    }";
    let conn = tracker_sparql::Connection::new().expect("Conn err");
    let mut cursor = conn.query(query.to_string()).expect("Query error or returns none");

    println!("Querying...");
    while let Some(c) = cursor.next() {
        println!("Result {:?}", c);
    }
}
