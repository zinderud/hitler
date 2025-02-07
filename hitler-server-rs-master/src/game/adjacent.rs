/* players_are_adjacent fonksiyonu, üç parametre alır: i, j ve n. Bu parametreler sırasıyla iki oyuncunun indekslerini (i ve j) ve toplam oyuncu sayısını (n) temsil eder. Fonksiyonun amacı, iki oyuncunun oyun tahtasında bitişik olup olmadığını belirlemektir.

Fonksiyonun içindeki adımlar şu şekildedir:

let diff = (n + i - j) % n;: Bu satır, i ve j oyuncularının indeksleri arasındaki farkı hesaplar. Modül (%) operatörü, döngüsel bir yapı oluşturmak için kullanılır. Böylece oyuncuların listesi dairesel olarak kabul edilir.
diff == 1 || diff == n - 1: Bu ifade, iki durumun doğruluğunu kontrol eder:
diff == 1: i oyuncusu, j oyuncusunun hemen yanında.
diff == n - 1: j oyuncusu, i oyuncusunun hemen yanında (döngüsel yapı nedeniyle).
Eğer bu iki durumdan biri doğruysa, fonksiyon true döner ve oyuncuların bitişik olduğunu belirtir. Aksi halde false döner. */


pub fn players_are_adjacent(i: usize, j: usize, n: usize) -> bool {
    let diff = (n + i - j) % n;
    diff == 1 || diff == n - 1
}
