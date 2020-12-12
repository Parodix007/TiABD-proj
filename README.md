# TiABD-proj
Testowanie szybkosci MariaDB/MySQL
* Aplikacja do testowania zapytan w MariaDB/MySQL napisana w jezyku Rust.
## Opis projektu
* Problemem tego projektu jest szybkosc wykonywania zapytan w bazach danych opartych o MariaDB/MySQL. Testowane jest to na przykladzie bazy danych [Employees z dokumentacji MySQL](https://github.com/datacharmer/test_db). Bedzie na niej przeprowadzane **2 testy** opart o konkretne problemy.
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
## Dzialanie algorytmu
* Podpunkt drugi z problemu "Wplyw indeksow na szybkosc przeszukiwania" nalezy przeprowadzic tylko **raz** (wedlug kryteriow badawczych). Reszte zapytan mozna przeprowadzic *n* liczbe razy. Aby moc przeprowadzic testy na swojej wersji bazy danych employees (lecz z niezmionymi polami ktore podlegaja badaniu) nalezy w pliku **database.rs** podmiec *conn_url* (linijka 12) na swoj, nastepnie uruchomic program i poczekac na wynik. Liczbe n oczywiscie tez mozna zmienic na swoja w pliku *main.rs* w linijce 5.
* Program posiada funkcje ktora nawiazuje polaczenie z baza danych i wykonuje okreslone zapytanie, do uzyskania szybkosci danego zapytania uzywam ``set profiling = 1`` ktore pozwala mi na zmierzenie kazdego zapytania z osobna dzieki czemu jestem w stanie uzyskac srednia. Po wykonaniu zapytania ustawiam wartosci *profiling* na zero i czyszcze wpisy w tabeli *profiles* za pomoca ``SET @@profiling_history_size = 0``.  
