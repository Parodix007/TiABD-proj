# TiABD-proj
Testowanie szybkosci MariaDB/MySQL
* Aplikacja do testowania zapytan w MariaDB/MySQL napisana w jezyku Rust.
## Opis projektu
* Problemem tego projektu jest szybkosc wykonywania zapytan w bazach danych opartych o MariaDB/MySQL. Testowane jest to na przykladzie bazy danych [Employees z dokumentacji MySQL](https://github.com/datacharmer/test_db). Bedzie na niej przeprowadzane 4-5 testow opartych o konkretne problemy.
## Problemy badawcze
* Wplyw indeksow na szybkosc przeszukiwania:
	* wykonać zapytanie do bazy danych, sprawdzające, ile zostało dokonanych wypłat na daną kwotę (czyli przez grupowanie),
	* utworzyć indeks na kolumnie salary (mierząc czas wykonywania polecenia SQL tworzącego go),
	* powtórzyć zapytanie.
* Uproszczenie silnika:
	* czyli np. czy jeżeli zrobimy grupowanie, ale wybierzemy tylko ``select *``, to czy to wpływa na czas działania - jeśli tak, i jest krócej, to znaczy, że silnik upraszcza zapytania itd.
## Srodowisko badawcze
* Program ktory odpowiada za wykonywanie operacji na bazie danych jest to [MySQL Workbench](https://www.mysql.com/products/workbench/).
* Do przeprowadzenia testow bede uzywal jezyka *Rust* ktory pozwoli mi wykonac *n* zapytan do bazy aby mozna bylo nastepnie pobrac sredni wynik.
