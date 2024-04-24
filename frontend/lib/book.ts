export function getBooks() {
    const books = fetch('http://localhost:8080/books')
      .then(response => response.text());

    return books;
}