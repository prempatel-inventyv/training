# SQL Movie Database Exercises

## 1. [Question]

**Question:** Write a SQL query to find the name and year of the movies. Return movie title, movie release year.

**ANSWER:** SELECT mov_title,mov_year from movie;

**OUTPUT:**

| mov_title                | mov_year |
| ------------------------ | -------- |
| Vertigo                  | 1958     |
| The Innocents            | 1961     |
| Lawrence of Arabia       | 1962     |
| The Deer Hunter          | 1978     |
| Amadeus                  | 1984     |
| Blade Runner             | 1982     |
| Eyes Wide Shut           | 1999     |
| The Usual Suspects       | 1995     |
| Chinatown                | 1974     |
| Boogie Nights            | 1997     |
| Annie Hall               | 1977     |
| Princess Mononoke        | 1997     |
| The Shawshank Redemption | 1994     |
| American Beauty          | 1999     |
| Titanic                  | 1997     |
| Good Will Hunting        | 1997     |
| Deliverance              | 1972     |
| Trainspotting            | 1996     |
| The Prestige             | 2006     |
| Donnie Darko             | 2001     |
| Slumdog Millionaire      | 2008     |
| Aliens                   | 1986     |
| Beyond the Sea           | 2004     |
| Avatar                   | 2009     |
| Seven Samurai            | 1954     |
| Spirited Away            | 2001     |
| Back to the Future       | 1985     |
| Braveheart               | 1995     |

---

## 2. [Question]

**Question:** Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.

**ANSWER:** SELECT mov_year from movie where mov_title = 'American Beauty'

**OUTPUT:**
| mov_year |
| -------- |
| 1999 |

---

## 3. [Question]

**Question:** Write a SQL query to find the movie that was released in 1999. Return movie title.

**ANSWER:** SELECT mov_title from movie where mov_year = 1999

**OUTPUT:**

| mov_title       |
| --------------- |
| Eyes Wide Shut  |
| American Beauty |

---

## 4. [Question]

**Question:** Write a SQL query to find those movies, which were released before 1998. Return movie title.

**ANSWER:** SELECT mov_title from movie where mov_year > 1998

**OUTPUT:**

| mov_title           |
| ------------------- |
| Eyes Wide Shut      |
| American Beauty     |
| The Prestige        |
| Donnie Darko        |
| Slumdog Millionaire |
| Beyond the Sea      |
| Avatar              |
| Spirited Away       |

---

## 5. [Question]

**Question:** Write a SQL query to find the name of all reviewers and movies together in a single list.

**ANSWER:** SELECT rev_name AS name
FROM movie_reviewer UNION SELECT mov_title AS name
FROM movie;

**OUTPUT:**
| name |
| ------------------------ |
| null |
| Seven Samurai |
| Avatar |
| Brandt Sponseller |
| American Beauty |
| Krug Stillo |
| Deliverance |
| Back to the Future |
| Victor Woeltjen |
| Wesley S. Walker |
| Jack Malvern |
| Princess Mononoke |
| Braveheart |
| Vertigo |
| Donnie Darko |
| Amadeus |
| Scott LeBrun |
| Blade Runner |
| Slumdog Millionaire |
| The Innocents |
| Alec Shaw |
| The Shawshank Redemption |
| Lawrence of Arabia |
| The Usual Suspects |
| The Deer Hunter |
| Josh Cates |
| Flagrant Baronessa |
| The Prestige |
| Spirited Away |
| Annie Hall |
| Hannah Steele |
| Titanic |
| Righty Sock |
| Sasha Goldshtein |
| Neal Wruck |
| Paul Monks |
| Aliens |
| Vincent Cadena |
| Beyond the Sea |
| Eyes Wide Shut |
| Chinatown |
| Boogie Nights |
| Richard Adams |
| Mike Salvati |
| Good Will Hunting |
| Trainspotting |
| Simon Wright |

---

## 6. [Question]

**Question:** Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.

**ANSWER:** SELECT DISTINCT movie_reviewer.rev_name FROM movie_reviewer JOIN movie_rating ON movie_reviewer.rev_id = movie_rating.rev_id WHERE movie_rating.rev_stars >= 7 ;

**OUTPUT:**

| rev_name           |
| ------------------ |
| null               |
| Hannah Steele      |
| Righty Sock        |
| Sasha Goldshtein   |
| Jack Malvern       |
| Vincent Cadena     |
| Brandt Sponseller  |
| Flagrant Baronessa |
| Krug Stillo        |
| Mike Salvati       |
| Victor Woeltjen    |
| Simon Wright       |

---

## 7. [Question]

**Question:** Write a SQL query to find the movies without any rating. Return movie title.

**ANSWER:** SELECT m.mov_title FROM movie m LEFT JOIN movie_rating mr ON m.mov_id = mr.mov_id WHERE mr.rev_stars IS NULL;

**OUTPUT:**

| mov_title                |
| ------------------------ |
| Chinatown                |
| Trainspotting            |
| Spirited Away            |
| Back to the Future       |
| Deliverance              |
| The Shawshank Redemption |
| Amadeus                  |
| The Prestige             |
| Braveheart               |
| The Deer Hunter          |
| Eyes Wide Shut           |

---

## 8. [Question]

**Question:** Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.

**ANSWER:** SELECT mov_title FROM movie WHERE mov_id = 905 or mov_id = 907 or mov_id = 917

**OUTPUT:**

| mov_title |

---

## 9. [Question]

**Question:** Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.

**ANSWER:** SELECT mov_title, mov_id, mov_year FROM movie WHERE mov_title = 'Boogie Nights' ORDER BY mov_year ASC;

**OUTPUT:**

| mov_title     | mov_id | mov_year |
| ------------- | ------ | -------- |
| Boogie Nights | 10     | 1997     |

---

## 10. [Question]

**Question:** Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.

**ANSWER:** SELECT act_id from actor where act_fname = 'Woody' and acrt_lname = 'Allen'

**OUTPUT:**

| act_id |
| ------ |
| 11     |

---

## 11. [Question]

**Question:** Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.

**ANSWER:** SELECT actor.\* FROM actor JOIN movie_cast ON actor.act_id = movie_cast.act_id JOIN movie ON movie.mov_id = movie_cast.mov_id WHERE movie.mov_title = 'Annie Hall';

**OUTPUT:**

| act_id | act_fname | acrt_lname | act_gender |
| ------ | --------- | ---------- | ---------- |
| 11     | Woody     | Allen      | M          |

---

## 12. [Question]

**Question:** Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.

**ANSWER:** SELECT director.dir_fname, director.dir_lname FROM director JOIN movie_direction ON director.dir_id = movie_direction.dir_id JOIN movie ON movie.mov_id = movie_direction.mov_id WHERE movie.mov_title = 'Eyes Wide Shut';

**OUTPUT:**

| dir_fname | dir_lname |
| --------- | --------- |
| Stanley   | Kubrick   |

---

## 13. [Question]

**Question:** Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.

**ANSWER:** SELECT mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country FROM movie WHERE mov_rel_country <> 'UK';

**OUTPUT:**

| mov_title     | mov_year | mov_time | mov_dt_rel | mov_rel_country |
| ------------- | -------- | -------- | ---------- | --------------- |
| The Innocents | 1961     | 100      | 1962-02-19 | SW              |
| Annie Hall    | 1977     | 93       | 1977-04-20 | USA             |
| Seven Samurai | 1954     | 207      | 1954-04-26 | JP              |

---

## 14. [Question]

**Question:** Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.

**ANSWER:** SELECT movie.mov_title, movie.mov_year, movie.mov_dt_rel, director.dir_fname, director.dir_lname, actor.act_fname, actor.acrt_lname FROM movie JOIN movie_rating ON movie_rating.mov_id = movie.mov_id JOIN movie_reviewer ON movie_reviewer.rev_id = movie_rating.rev_id JOIN movie_direction ON movie_direction.mov_id = movie.mov_id JOIN director ON director.dir_id = movie_direction.dir_id JOIN movie_cast ON movie_cast.mov_id = movie.mov_id JOIN actor ON actor.act_id = movie_cast.act_id WHERE movie_reviewer.rev_name IS NULL;

**OUTPUT:**

| mov_title         | mov_year | mov_dt_rel | dir_fname | dir_lname | act_fname | acrt_lname |
| ----------------- | -------- | ---------- | --------- | --------- | --------- | ---------- |
| Blade Runner      | 1982     | 1982-09-09 | Ridley    | Scott     | Harrison  | Ford       |
| Princess Mononoke | 1997     | 2001-10-19 | Hayao     | Miyazaki  | Claire    | Danes      |

---

## 15. [Question]

**Question:** Write a SQL query to find those movies directed by the director whose first name is Woddy and last name is Allen. Return movie title.

**ANSWER:** SELECT movie.mov_title FROM movie JOIN movie_direction ON movie.mov_id = movie_direction.mov_id JOIN director ON director.dir_id = movie_direction.dir_id WHERE director.dir_fname = 'Woody' AND director.dir_lname = 'Allen';

**OUTPUT:**

| mov_title  |
| ---------- |
| Annie Hall |

---

## 16. [Question]

**Question:** Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.

**ANSWER:** SELECT movie.mov_year FROM movie JOIN movie_rating ON movie.mov_id = movie_rating.mov_id WHERE movie_rating.rev_stars >= 3 ORDER BY movie.mov_year ASC;

**OUTPUT:**

| mov_year |
| -------- |
| 1954     |
| 1958     |
| 1961     |
| 1962     |
| 1977     |
| 1982     |
| 1986     |
| 1995     |
| 1997     |
| 1997     |
| 1997     |
| 1997     |
| 1999     |
| 2001     |
| 2004     |
| 2008     |
| 2009     |

---

## 17. [Question]

**Question:** Write a SQL query to search for movies that do not have any ratings. Return movie title.

**ANSWER:** SELECT movie.mov_title FROM movie LEFT JOIN movie_rating ON movie.mov_id = movie_rating.mov_id WHERE movie_rating.mov_id IS NULL;

**OUTPUT:**

| mov_title                |
| ------------------------ |
| Spirited Away            |
| Back to the Future       |
| Deliverance              |
| The Shawshank Redemption |
| Amadeus                  |
| The Prestige             |
| Braveheart               |
| The Deer Hunter          |
| Eyes Wide Shut           |

---

## 18. [Question]

**Question:** Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.

**ANSWER:** SELECT DISTINCT movie_reviewer.rev_name FROM movie_reviewer JOIN movie_rating ON movie_reviewer.rev_id = movie_rating.rev_id WHERE movie_rating.rev_stars IS NULL;

**OUTPUT:**

| rev_name     |
| ------------ |
| Neal Wruck   |
| Scott LeBrun |

---

## 19. [Question]

**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.

**ANSWER:** SELECT movie_reviewer.rev_name, movie.mov_title, movie_rating.rev_stars FROM movie_rating JOIN movie_reviewer ON movie_reviewer.rev_id = movie_rating.rev_id JOIN movie ON movie.mov_id = movie_rating.mov_id WHERE movie_reviewer.rev_name IS NOT NULL AND movie_rating.rev_stars IS NOT NULL ORDER BY movie_reviewer.rev_name ASC, movie.mov_title ASC, movie_rating.rev_stars ASC;

**OUTPUT:**

| rev_name           | mov_title           | rev_stars |
| ------------------ | ------------------- | --------- |
| Brandt Sponseller  | Aliens              | 8.4       |
| Flagrant Baronessa | Lawrence of Arabia  | 8.3       |
| Hannah Steele      | Donnie Darko        | 8.1       |
| Jack Malvern       | The Innocents       | 7.9       |
| Josh Cates         | Good Will Hunting   | 4.0       |
| Krug Stillo        | Seven Samurai       | 7.7       |
| Mike Salvati       | Annie Hall          | 8.1       |
| Paul Monks         | Boogie Nights       | 3.0       |
| Richard Adams      | Beyond the Sea      | 6.7       |
| Righty Sock        | Titanic             | 7.7       |
| Righty Sock        | Vertigo             | 8.4       |
| Sasha Goldshtein   | American Beauty     | 7.0       |
| Simon Wright       | The Usual Suspects  | 8.6       |
| Victor Woeltjen    | Avatar              | 7.3       |
| Vincent Cadena     | Slumdog Millionaire | 8.0       |

---

## 20. [Question]

**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer's name, movie title. Return reviewer's name, movie title.

**ANSWER:**
SELECT
movie_reviewer.rev_name,
movie.mov_title
FROM movie_reviewer
JOIN movie_rating
ON movie_reviewer.rev_id = movie_rating.rev_id
JOIN movie
ON movie.mov_id = movie_rating.mov_id
WHERE movie_rating.rev_stars IS NOT NULL
GROUP BY movie_reviewer.rev_name, movie.mov_title
ORDER BY movie_reviewer.rev_name, movie.mov_title;

**OUTPUT:**

| rev_name           | mov_title           |
| ------------------ | ------------------- |
| Brandt Sponseller  | Aliens              |
| Flagrant Baronessa | Lawrence of Arabia  |
| Hannah Steele      | Donnie Darko        |
| Jack Malvern       | The Innocents       |
| Josh Cates         | Good Will Hunting   |
| Krug Stillo        | Seven Samurai       |
| Mike Salvati       | Annie Hall          |
| Paul Monks         | Boogie Nights       |
| Richard Adams      | Beyond the Sea      |
| Righty Sock        | Titanic             |
| Righty Sock        | Vertigo             |
| Sasha Goldshtein   | American Beauty     |
| Simon Wright       | The Usual Suspects  |
| Victor Woeltjen    | Avatar              |
| Vincent Cadena     | Slumdog Millionaire |
| null               | Blade Runner        |
| null               | Princess Mononoke   |

---

## 21. [Question]

**Question:** Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.

**ANSWER:**
SELECT movie.mov_title, MAX(movie_rating.rev_stars) AS max_stars
FROM movie
JOIN movie_rating ON movie.mov_id = movie_rating.mov_id
GROUP BY movie.mov_title
ORDER BY movie.mov_title;

**OUTPUT:**

| mov_title           | max_stars |
| ------------------- | --------- |
| Aliens              | 8.4       |
| American Beauty     | 7.0       |
| Annie Hall          | 8.1       |
| Avatar              | 7.3       |
| Beyond the Sea      | 6.7       |
| Blade Runner        | 8.2       |
| Boogie Nights       | 3.0       |
| Chinatown           | null      |
| Donnie Darko        | 8.1       |
| Good Will Hunting   | 4.0       |
| Lawrence of Arabia  | 8.3       |
| Princess Mononoke   | 8.4       |
| Seven Samurai       | 7.7       |
| Slumdog Millionaire | 8.0       |
| The Innocents       | 7.9       |
| The Usual Suspects  | 8.6       |
| Titanic             | 7.7       |
| Trainspotting       | null      |
| Vertigo             | 8.4       |

---

## 22. [Question]

**Question:** Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name.

**ANSWER:** SELECT DISTINCT mr.rev_name
FROM movie m
JOIN movie_rating r ON m.mov_id = r.mov_id
JOIN movie_reviewer mr ON r.rev_id = mr.rev_id
WHERE m.mov_title = 'American Beauty';

**OUTPUT:**

| rev_name         |
| ---------------- |
| Sasha Goldshtein |

---

## 23. [Question]

**Question:** Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.

**ANSWER:**
SELECT m.mov_title
FROM movie m
WHERE NOT EXISTS (
SELECT 1
FROM movie_rating r
JOIN movie_reviewer mr ON r.rev_id = mr.rev_id
WHERE r.mov_id = m.mov_id
AND mr.rev_name <> 'Paul Monks'
);

**OUTPUT:**

| mov_title                |
| ------------------------ |
| Spirited Away            |
| Back to the Future       |
| Deliverance              |
| Princess Mononoke        |
| Boogie Nights            |
| The Shawshank Redemption |
| Amadeus                  |
| The Prestige             |
| Braveheart               |
| Blade Runner             |
| The Deer Hunter          |
| Eyes Wide Shut           |

---

## 24. [Question]

**Question:** Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.

**ANSWER:**SELECT mr.rev_name,
m.mov_title,
r.rev_stars
FROM movie_rating r
JOIN movie m ON r.mov_id = m.mov_id
JOIN movie_reviewer mr ON r.rev_id = mr.rev_id
WHERE r.rev_stars = (
SELECT MIN(rev_stars)
FROM movie_rating
WHERE rev_stars IS NOT NULL
);

**OUTPUT:**

| rev_name   | mov_title     | rev_stars |
| ---------- | ------------- | --------- |
| Paul Monks | Boogie Nights | 3.0       |

---

## 25. [Question]

**Question:** Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.

**ANSWER:** SELECT m.mov_title
FROM movie m
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id
WHERE d.dir_fname = 'James'
AND d.dir_lname = 'Cameron';

**OUTPUT:**

| mov_title |
| --------- |
| Titanic   |
| Aliens    |

---

## 26. [Question]

**Question:** Write a query in SQL to find the movies in which one or more actors appeared in more than one film.

**ANSWER:** SELECT DISTINCT m.mov_title
FROM movie m
JOIN movie_cast mc ON m.mov_id = mc.mov_id
WHERE mc.act_id IN (
SELECT act_id
FROM movie_cast
GROUP BY act_id
HAVING COUNT(DISTINCT mov_id) > 1
);

**OUTPUT:**

| mov_title       |
| --------------- |
| American Beauty |
| Beyond the Sea  |

---

## 27. [Question]

**Question:**

Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.

**ANSWER:**

```sql
SELECT reviewer.rev_name
FROM movie_reviewer reviewer
JOIN movie_rating rating
  ON reviewer.rev_id = rating.rev_id
WHERE rating.rev_stars IS NULL;
```

**OUTPUT:**

| rev_name     |
| ------------ |
| Neal Wruck   |
| Scott LeBrun |

---

## 28. [Question]

**Question:**

Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.

**ANSWER:**

```sql
SELECT actor.act_fname,
       actor.acrt_lname,
       movie_cast.role
FROM actor
JOIN movie_cast
  ON actor.act_id = movie_cast.act_id
JOIN movie
  ON movie_cast.mov_id = movie.mov_id
WHERE movie.mov_title = 'Annie Hall';
```

**OUTPUT:**

| act_fname | acrt_lname | role        |
| --------- | ---------- | ----------- |
| Woody     | Allen      | Alvy Singer |

---

## 29. [Question]

**Question:**

Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.

**ANSWER:**

```sql
SELECT director.dir_fname,
       director.dir_lname,
       movie.mov_title
FROM director
JOIN movie_direction
  ON movie_direction.dir_id = director.dir_id
JOIN movie
  ON movie.mov_id = movie_direction.mov_id
WHERE movie.mov_title = 'Eyes Wide Shut';
```

**OUTPUT:**

| dir_fname | dir_lname | mov_title      |
| --------- | --------- | -------------- |
| Stanley   | Kubrick   | Eyes Wide Shut |

---

## 30. [Question]

**Question:**

Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.

**ANSWER:**

```sql
SELECT director.dir_fname,
       director.dir_lname,
       movie.mov_title
FROM director
JOIN movie_direction
  ON movie_direction.dir_id = director.dir_id
JOIN movie
  ON movie.mov_id = movie_direction.mov_id
JOIN movie_cast
  ON movie.mov_id = movie_cast.mov_id
WHERE movie_cast.role = 'Sean Maguire';
```

**OUTPUT:**

| dir_fname | dir_lname | mov_title         |
| --------- | --------- | ----------------- |
| Gus       | Van Sant  | Good Will Hunting |

---

## 31. [Question]

**Question:**

Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.

**ANSWER:**

```sql
SELECT actor.act_fname,
       actor.acrt_lname,
       movie.mov_title,
       movie.mov_year
FROM actor
JOIN movie_cast
  ON actor.act_id = movie_cast.act_id
JOIN movie
  ON movie.mov_id = movie_cast.mov_id
WHERE movie.mov_year < 1989
   OR movie.mov_year > 2001;
```

**OUTPUT:**

| act_fname | acrt_lname | mov_title           | mov_year |
| --------- | ---------- | ------------------- | -------- |
| James     | Stewart    | Vertigo             | 1958     |
| Deborah   | Kerr       | The Innocents       | 1961     |
| Peter     | OToole     | Lawrence of Arabia  | 1962     |
| Robert    | De Niro    | The Deer Hunter     | 1978     |
| F. Murray | Abraham    | Amadeus             | 1984     |
| Harrison  | Ford       | Blade Runner        | 1982     |
| Woody     | Allen      | Annie Hall          | 1977     |
| Jon       | Voight     | Deliverance         | 1972     |
| Dev       | Patel      | Slumdog Millionaire | 2008     |
| Sigourney | Weaver     | Aliens              | 1986     |
| Kevin     | Spacey     | Beyond the Sea      | 2004     |
| Jack      | Nicholson  | Chinatown           | 1974     |
| Christian | Bale       | Chinatown           | 1974     |

---

## 32. [Question]

**Question:**

Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.

**ANSWER:**

```sql
SELECT director.dir_fname,
       director.dir_lname,
       COUNT(DISTINCT genres.gen_title) AS no_of_movies
FROM director
JOIN movie_direction
  ON movie_direction.dir_id = director.dir_id
JOIN movie_genres
  ON movie_genres.mov_id = movie_direction.mov_id
JOIN genres
  ON genres.gen_id = movie_genres.gen_id
GROUP BY director.dir_fname,
         director.dir_lname,
         genres.gen_title
ORDER BY director.dir_fname,
         director.dir_lname;
```

**OUTPUT:**

| dir_fname | dir_lname | no_of_movies |
| --------- | --------- | ------------ |
| Alfred    | Hitchcock | 1            |
| Bryan     | Singer    | 1            |
| Danny     | Boyle     | 1            |
| David     | Lean      | 1            |
| Frank     | Darabont  | 1            |
| Hayao     | Miyazaki  | 1            |
| Jack      | Clayton   | 1            |
| James     | Cameron   | 1            |
| John      | Boorman   | 1            |
| Kevin     | Spacey    | 1            |
| Michael   | Cimino    | 1            |
| Ridley    | Scott     | 1            |
| Sam       | Mendes    | 1            |
| Stanley   | Kubrick   | 1            |
| Woody     | Allen     | 1            |

---

## 33. [Question]

**Question:**

Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.

**ANSWER:**

```sql
SELECT movie.mov_title,
       movie.mov_year,
       genres.gen_title
FROM movie
JOIN movie_genres
  ON movie.mov_id = movie_genres.mov_id
JOIN genres
  ON genres.gen_id = movie_genres.gen_id;
```

**OUTPUT:**

| mov_title                | mov_year | gen_title |
| ------------------------ | -------- | --------- |
| Aliens                   | 1986     | Action    |
| Deliverance              | 1972     | Adventure |
| Lawrence of Arabia       | 1962     | Adventure |
| Princess Mononoke        | 1997     | Animation |
| Annie Hall               | 1977     | Comedy    |
| The Usual Suspects       | 1995     | Crime     |
| The Shawshank Redemption | 1994     | Crime     |
| Spirited Away            | 2001     | Drama     |
| Braveheart               | 1995     | Drama     |
| Trainspotting            | 1996     | Drama     |
| Slumdog Millionaire      | 2008     | Drama     |
| The Innocents            | 1961     | Horror    |
| Beyond the Sea           | 2004     | Music     |
| Eyes Wide Shut           | 1999     | Mystery   |
| Back to the Future       | 1985     | Mystery   |
| Vertigo                  | 1958     | Mystery   |
| American Beauty          | 1999     | Romance   |
| Blade Runner             | 1982     | Thriller  |
| The Deer Hunter          | 1978     | War       |

---

## 34. [Question]

**Question:**

Write a SQL query to find all the movies with year, genres, and name of the director.

**ANSWER:**

```sql
SELECT movie.mov_title,
       movie.mov_year,
       genres.gen_title,
       director.dir_fname,
       director.dir_lname
FROM movie
JOIN movie_genres
  ON movie.mov_id = movie_genres.mov_id
JOIN genres
  ON genres.gen_id = movie_genres.gen_id
JOIN movie_direction
  ON movie.mov_id = movie_direction.mov_id
JOIN director
  ON director.dir_id = movie_direction.dir_id;
```

**OUTPUT:**

| mov_title                | mov_year | gen_title | dir_fname | dir_lname |
| ------------------------ | -------- | --------- | --------- | --------- |
| Aliens                   | 1986     | Action    | James     | Cameron   |
| Deliverance              | 1972     | Adventure | John      | Boorman   |
| Lawrence of Arabia       | 1962     | Adventure | David     | Lean      |
| Princess Mononoke        | 1997     | Animation | Hayao     | Miyazaki  |
| Annie Hall               | 1977     | Comedy    | Woody     | Allen     |
| The Usual Suspects       | 1995     | Crime     | Bryan     | Singer    |
| The Shawshank Redemption | 1994     | Crime     | Frank     | Darabont  |
| Trainspotting            | 1996     | Drama     | Danny     | Boyle     |
| Slumdog Millionaire      | 2008     | Drama     | Danny     | Boyle     |
| The Innocents            | 1961     | Horror    | Jack      | Clayton   |
| Beyond the Sea           | 2004     | Music     | Kevin     | Spacey    |
| Eyes Wide Shut           | 1999     | Mystery   | Stanley   | Kubrick   |
| Vertigo                  | 1958     | Mystery   | Alfred    | Hitchcock |
| American Beauty          | 1999     | Romance   | Sam       | Mendes    |
| Blade Runner             | 1982     | Thriller  | Ridley    | Scott     |
| The Deer Hunter          | 1978     | War       | Michael   | Cimino    |

---

## 35. [Question]

**Question:** Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.

**ANSWER:**

```sql
SELECT movie.mov_title, movie.mov_dt_rel, movie.mov_year, movie.mov_lang, movie.mov_time,
       director.dir_fname, director.dir_lname
FROM movie
JOIN movie_direction ON movie_direction.mov_id = movie.mov_id
JOIN director ON director.dir_id = movie_direction.dir_id
WHERE mov_dt_rel < '1989-01-01'
ORDER BY mov_dt_rel DESC;
```

**OUTPUT:**
| mov_title | mov_dt_rel | mov_year | mov_lang | mov_time | dir_fname | dir_lname |
|--------------------|------------|----------|----------|----------|-----------|-----------|
| Aliens | 1986-08-29 | 1986 | English | 137 | James | Cameron |
| Amadeus | 1985-01-07 | 1984 | English | 160 | Milos | Forman |
| Deliverance | 1982-10-05 | 1972 | English | 109 | John | Boorman |
| Blade Runner | 1982-09-09 | 1982 | English | 117 | Ridley | Scott |
| The Deer Hunter | 1979-03-08 | 1978 | English | 183 | Michael | Cimino |
| Annie Hall | 1977-04-20 | 1977 | English | 93 | Woody | Allen |
| Chinatown | 1974-08-09 | 1974 | English | 130 | Roman | Polanski |
| Lawrence of Arabia | 1962-12-11 | 1962 | English | 216 | David | Lean |
| The Innocents | 1962-02-19 | 1961 | English | 100 | Jack | Clayton |
| Vertigo | 1958-08-24 | 1958 | English | 128 | Alfred | Hitchcock |

---

## 36. [Question]

**Question:** Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.

**ANSWER:**

```sql
SELECT AVG(movie.mov_time) AS AVERAGE_TIME, genres.gen_title,
       COUNT(gen_title) AS number_of_movie
FROM movie
JOIN movie_genres ON movie.mov_id = movie_genres.mov_id
JOIN genres ON genres.gen_id = movie_genres.gen_id
GROUP BY genres.gen_title;
```

**OUTPUT:**
| average_time | gen_title | number_of_movie |
|----------------------|-----------|-----------------|
| 117.0000000000000000 | Thriller | 1 |
| 122.0000000000000000 | Romance | 1 |
| 162.5000000000000000 | Adventure | 2 |
| 93.0000000000000000 | Comedy | 1 |
| 134.0000000000000000 | Animation | 1 |
| 124.0000000000000000 | Crime | 2 |
| 118.0000000000000000 | Music | 1 |
| 100.0000000000000000 | Horror | 1 |
| 129.2500000000000000 | Drama | 4 |
| 183.0000000000000000 | War | 1 |
| 134.3333333333333333 | Mystery | 3 |
| 137.0000000000000000 | Action | 1 |

---

## 37. [Question]

**Question:** Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.

**ANSWER:**

```sql
SELECT movie.mov_title, movie.mov_year, director.dir_fname, director.dir_lname,
       actor.act_fname, actor.acrt_lname, movie_cast.role
FROM movie
JOIN movie_direction ON movie_direction.mov_id = movie.mov_id
JOIN director ON director.dir_id = movie_direction.dir_id
JOIN movie_cast ON movie_cast.mov_id = movie.mov_id
JOIN actor ON actor.act_id = movie_cast.act_id
ORDER BY mov_time ASC
LIMIT 1;
```

**OUTPUT:**
| mov_title | mov_year | dir_fname | dir_lname | act_fname | acrt_lname | role |
|------------|----------|-----------|-----------|-----------|------------|-------------|
| Annie Hall | 1977 | Woody | Allen | Woody | Allen | Alvy Singer |

---

## 38. [Question]

**Question:** Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.

**ANSWER:**

```sql
SELECT movie.mov_year, movie_rating.rev_stars
FROM movie
JOIN movie_rating ON movie.mov_id = movie_rating.mov_id
WHERE movie_rating.rev_stars = 3 OR movie_rating.rev_stars = 4
ORDER BY movie.mov_year ASC;
```

**OUTPUT:**
| mov_year | rev_stars |
|----------|-----------|
| 1997 | 3.0 |
| 1997 | 4.0 |

---

## 39. [Question]

**Question:** Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.

**ANSWER:**

```sql
SELECT movie_reviewer.rev_name, movie.mov_title, movie_rating.rev_stars
FROM movie
JOIN movie_rating ON movie_rating.mov_id = movie.mov_id
JOIN movie_reviewer ON movie_reviewer.rev_id = movie_rating.rev_id;
```

**OUTPUT:**
| rev_name | mov_title | rev_stars |
|--------------------|---------------------|-----------|
| Righty Sock | Vertigo | 8.4 |
| Jack Malvern | The Innocents | 7.9 |
| Flagrant Baronessa | Lawrence of Arabia | 8.3 |
| null | Blade Runner | 8.2 |
| Victor Woeltjen | Avatar | 7.3 |
| Simon Wright | The Usual Suspects | 8.6 |
| Neal Wruck | Chinatown | null |
| Paul Monks | Boogie Nights | 3.0 |
| Mike Salvati | Annie Hall | 8.1 |
| null | Princess Mononoke | 8.4 |
| Sasha Goldshtein | American Beauty | 7.0 |
| Righty Sock | Titanic | 7.7 |
| Josh Cates | Good Will Hunting | 4.0 |
| Krug Stillo | Seven Samurai | 7.7 |
| Scott LeBrun | Trainspotting | null |
| Hannah Steele | Donnie Darko | 8.1 |
| Vincent Cadena | Slumdog Millionaire | 8.0 |
| Brandt Sponseller | Aliens | 8.4 |
| Richard Adams | Beyond the Sea | 6.7 |

---

## 40. [Question]

**Question:** Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.

**ANSWER:**

```sql
SELECT movie.mov_title, MAX(movie_rating.rev_stars) AS movie_rating
FROM movie
JOIN movie_rating ON movie.mov_id = movie_rating.mov_id
GROUP BY movie.mov_title
ORDER BY movie.mov_title;
```

**OUTPUT:**
| mov_title | movie_rating |
|---------------------|--------------|
| Aliens | 8.4 |
| American Beauty | 7.0 |
| Annie Hall | 8.1 |
| Avatar | 7.3 |
| Beyond the Sea | 6.7 |
| Blade Runner | 8.2 |
| Boogie Nights | 3.0 |
| Chinatown | null |
| Donnie Darko | 8.1 |
| Good Will Hunting | 4.0 |
| Lawrence of Arabia | 8.3 |
| Princess Mononoke | 8.4 |
| Seven Samurai | 7.7 |
| Slumdog Millionaire | 8.0 |
| The Innocents | 7.9 |
| The Usual Suspects | 8.6 |
| Titanic | 7.7 |
| Trainspotting | null |
| Vertigo | 8.4 |

---

## 41. [Question]

**Question:** Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.

**ANSWER:**

```sql
SELECT movie.mov_title, movie_rating.rev_stars, director.dir_fname, director.dir_lname
FROM movie
JOIN movie_rating ON movie.mov_id = movie_rating.mov_id
JOIN movie_direction ON movie_direction.mov_id = movie.mov_id
JOIN director ON director.dir_id = movie_direction.dir_id
WHERE movie_rating.rev_stars IS NOT NULL;
```

**OUTPUT:**
| mov_title | rev_stars | dir_fname | dir_lname |
|---------------------|-----------|-----------|-----------------|
| Vertigo | 8.4 | Alfred | Hitchcock |
| The Innocents | 7.9 | Jack | Clayton |
| Lawrence of Arabia | 8.3 | David | Lean |
| Blade Runner | 8.2 | Ridley | Scott |
| The Usual Suspects | 8.6 | Bryan | Singer |
| Boogie Nights | 3.0 | Paul | Thomas Anderson |
| Annie Hall | 8.1 | Woody | Allen |
| Princess Mononoke | 8.4 | Hayao | Miyazaki |
| American Beauty | 7.0 | Sam | Mendes |
| Titanic | 7.7 | James | Cameron |
| Good Will Hunting | 4.0 | Gus | Van Sant |
| Donnie Darko | 8.1 | Richard | Kelly |
| Slumdog Millionaire | 8.0 | Danny | Boyle |
| Aliens | 8.4 | James | Cameron |
| Beyond the Sea | 6.7 | Kevin | Spacey |

---

## 42. [Question]

**Question:** Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.

**ANSWER:**

**OUTPUT:**

---

## 43. [Question]

**Question:** Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.

**ANSWER:**

```sql
SELECT movie.mov_title, director.dir_fname, director.dir_lname,
       actor.act_fname, actor.acrt_lname
FROM movie
JOIN movie_cast ON movie.mov_id = movie_cast.mov_id
JOIN actor ON movie_cast.act_id = actor.act_id
JOIN movie_direction ON movie.mov_id = movie_direction.mov_id
JOIN director ON director.dir_id = movie_direction.dir_id;
```

**OUTPUT:**
| mov_title | dir_fname | dir_lname | act_fname | acrt_lname |
|--------------------------|-----------|-----------------|-----------|------------|
| Vertigo | Alfred | Hitchcock | James | Stewart |
| The Innocents | Jack | Clayton | Deborah | Kerr |
| Lawrence of Arabia | David | Lean | Peter | OToole |
| The Deer Hunter | Michael | Cimino | Robert | De Niro |
| Amadeus | Milos | Forman | F. Murray | Abraham |
| Blade Runner | Ridley | Scott | Harrison | Ford |
| Eyes Wide Shut | Stanley | Kubrick | Nicole | Kidman |
| The Usual Suspects | Bryan | Singer | Stephen | Baldwin |
| Chinatown | Roman | Polanski | Christian | Bale |
| Chinatown | Roman | Polanski | Jack | Nicholson |
| Boogie Nights | Paul | Thomas Anderson | Mark | Wahlberg |
| Annie Hall | Woody | Allen | Woody | Allen |
| Princess Mononoke | Hayao | Miyazaki | Claire | Danes |
| The Shawshank Redemption | Frank | Darabont | Tim | Robbins |
| American Beauty | Sam | Mendes | Kevin | Spacey |
| Titanic | James | Cameron | Kate | Winslet |
| Good Will Hunting | Gus | Van Sant | Robin | Williams |
| Deliverance | John | Boorman | Jon | Voight |
| Trainspotting | Danny | Boyle | Ewan | McGregor |
| Donnie Darko | Richard | Kelly | Maggie | Gyllenhaal |
| Slumdog Millionaire | Danny | Boyle | Dev | Patel |
| Aliens | James | Cameron | Sigourney | Weaver |
| Beyond the Sea | Kevin | Spacey | Kevin | Spacey |

---

## 44. [Question]

**Question:** Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.

**ANSWER:**

```sql
SELECT movie.mov_title, actor.act_fname, actor.acrt_lname, movie_cast.role
FROM movie
JOIN movie_direction ON movie.mov_id = movie_direction.mov_id
JOIN director ON director.dir_id = movie_direction.dir_id
JOIN movie_cast ON movie_cast.mov_id = movie.mov_id
JOIN actor ON actor.act_id = movie_cast.act_id
WHERE actor.act_fname = director.dir_fname
  AND actor.acrt_lname = director.dir_lname;
```

**OUTPUT:**
| mov_title | act_fname | acrt_lname | role |
|----------------|-----------|------------|-------------|
| Annie Hall | Woody | Allen | Alvy Singer |
| Beyond the Sea | Kevin | Spacey | Bobby Darin |

---

## 45. [Question]

**Question:** Write a SQL query to find the cast list of the movie 'Chinatown'. Return first name, last name.

**ANSWER:**

```sql
SELECT actor.act_fname, actor.acrt_lname
FROM movie
JOIN movie_cast ON movie.mov_id = movie_cast.mov_id
JOIN actor ON actor.act_id = movie_cast.act_id
WHERE movie.mov_title = 'Chinatown';
```

**OUTPUT:**
| act_fname | acrt_lname |
|-----------|------------|
| Jack | Nicholson |
| Christian | Bale |

---

## 46. [Question]

**Question:** Write a SQL query to find those movies where actor's first name is 'Harrison' and last name is 'Ford'. Return movie title.

**ANSWER:**

```sql
SELECT movie.mov_title
FROM movie
JOIN movie_cast ON movie_cast.mov_id = movie.mov_id
JOIN actor ON actor.act_id = movie_cast.act_id
WHERE actor.act_fname = 'Harrison'
  AND actor.acrt_lname = 'Ford';
```

**OUTPUT:**
| mov_title |
|--------------|
| Blade Runner |

---

## 47. [Question]

**Question:** Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.

**ANSWER:**

```sql
SELECT movie.mov_title, movie.mov_year, movie_rating.rev_stars, movie.mov_rel_country
FROM movie
JOIN movie_rating ON movie_rating.mov_id = movie.mov_id
WHERE movie_rating.rev_stars IS NOT NULL
ORDER BY movie_rating.rev_stars DESC
LIMIT 1;
```

**OUTPUT:**
| mov_title | mov_year | rev_stars | mov_rel_country |
|--------------------|----------|-----------|-----------------|
| The Usual Suspects | 1995 | 8.6 | UK |

---

## 48. [Question]

**Question:** Write a SQL query to find the highest-rated 'Mystery Movies'. Return the title, year, and rating.

**ANSWER:**

```sql
SELECT movie.mov_title, movie.mov_year, movie_rating.rev_stars, genres.gen_title
FROM movie
JOIN movie_rating ON movie.mov_id = movie_rating.mov_id
JOIN movie_genres ON movie_genres.mov_id = movie.mov_id
JOIN genres ON genres.gen_id = movie_genres.gen_id
WHERE genres.gen_title = 'Mystery'
ORDER BY movie_rating.rev_stars DESC
LIMIT 1;
```

**OUTPUT:**
| mov_title | mov_year | rev_stars | gen_title |
|-----------|----------|-----------|-----------|
| Vertigo | 1958 | 8.4 | Mystery |

---

## 49. [Question]

**Question:** Write a SQL query to find the years when most of the 'Mystery Movies' produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.

**ANSWER:**

```sql
SELECT movie.mov_year
FROM movie
JOIN movie_genres ON movie_genres.mov_id = movie.mov_id
JOIN genres ON genres.gen_id = movie_genres.gen_id
JOIN movie_rating ON movie_rating.mov_id = movie.mov_id
WHERE genres.gen_title = 'Mystery'
  AND movie_rating.rev_stars IS NOT NULL
GROUP BY movie.mov_year
ORDER BY COUNT(*) DESC
LIMIT 1;
```

**OUTPUT:**
| mov_year |
|----------|
| 1958 |

---

## 50. [Question]

**Question:** Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

**ANSWER:**

```sql
SELECT movie.mov_title, actor.act_fname, actor.acrt_lname, movie.mov_year,
       movie_cast.role, genres.gen_title, director.dir_fname, director.dir_lname,
       movie.mov_dt_rel, movie_rating.rev_stars
FROM movie
JOIN movie_cast ON movie_cast.mov_id = movie.mov_id
JOIN actor ON movie_cast.act_id = actor.act_id
JOIN movie_genres ON movie.mov_id = movie_genres.mov_id
JOIN genres ON genres.gen_id = movie_genres.gen_id
JOIN movie_direction ON movie_direction.mov_id = movie.mov_id
JOIN director ON movie_direction.dir_id = director.dir_id
LEFT JOIN movie_rating ON movie_rating.mov_id = movie.mov_id
WHERE actor.act_gender = 'F';
```

**OUTPUT:**
| mov_title | act_fname | acrt_lname | mov_year | role | gen_title | dir_fname | dir_lname | mov_dt_rel | rev_stars |
|-------------------|-----------|------------|----------|---------------|-----------|-----------|-----------|------------|-----------|
| The Innocents | Deborah | Kerr | 1961 | Miss Giddens | Horror | Jack | Clayton | 1962-02-19 | 7.9 |
| Princess Mononoke | Claire | Danes | 1997 | San | Animation | Hayao | Miyazaki | 2001-10-19 | 8.4 |
| Aliens | Sigourney | Weaver | 1986 | Ripley | Action | James | Cameron | 1986-08-29 | 8.4 |
| Eyes Wide Shut | Nicole | Kidman | 1999 | Alice Harford | Mystery | Stanley | Kubrick | null | null |
