"use client";

import React, { useEffect, useState } from 'react';

const Home = () => {
  const [books, setBooks] = useState("");

  useEffect(() => {

  }, []);

  return (
    <div>
      <h1>Books</h1>
      <p>{books}</p>
    </div>
  );
};

export default Home;