#![allow(non_snake_case)]
struct Test(i32);//intero che vive in oggetto di tipo Test
impl Drop for Test{
    //Lo decoro con un tratto
    fn drop(&mut self) {
        //chiamato drop come metodo come testamento quando muoiono oggetti
        //metodo che il compilatore invoca prima di rilasciare un pezzo di memoria
        println!("Destroying  Test({}) at address : {:p}", self.0,self);
        //oggetto sta nello stack e distrutto alla fine prima di return
    }

}

fn main() {
    for i in  0..4 {
        println!("Iterazione # {}",i);
        let t = Test(1);
        //t.drop(); non si può fare
        println!("Created Test ({}) at address {:p}",t.0,&t);
        println!("Finishing...");

        /*ogni volta crea leva crea leva allargandosi e richiudendosi*/
        /*variabili locali iniziano nel punto in cui le introducono e cessano di esistere
        dopo la {}
         */

    }

    return;


    //let mut v:Vec<u8> = Vec::<u8>::new(); //parte vuoto
    //qua risposta = 0x04;
    //interi a 32 bit solo a multipli di 4 , interi a 64 solo multipli di 9

    //mi dice a cosa sta puntando il mio puntatore
    //restituisce puntatore nativo in sola lettura (lo stampo come numero)
    // let mut v:Vec<u8> = Vec::<u8>::new();
    //println!("ptr: {:p}, capacity:{}, len: {}", v.as_ptr(),v.capacity(),v.len() );
    //qua la risposta è 0x01

    //let mut v:Vec<String> = Vec::<String>::new();
    //println!("ptr: {:p}, capacity:{}, len: {}", v.as_ptr(),v.capacity(),v.len() );
    //risposta è 0x08: l'allineamento deve essere a multipli di 8


    /*
    let mut v:Vec<i32> = Vec::<i32>::new(); //parte vuoto
    let mut v1:Vec<u8> = Vec::<u8>::with_capacity(8); //si può già allocare con una stima
    println!("ptr: {:p}, capacity:{},len: {}", v.as_ptr(),v.capacity(),v.len() );
    //println!("ptr v1: {:p}, capacity:{},len: {}", v1.as_ptr(),v1.capacity(),v1.len() );
    v.push(5); //nuovo dato
    //prima capacity  e len 0;: illeciti
    println!("ptr: {:p}, capacity:{},len: {}", v.as_ptr(),v.capacity(),v.len() );
    println!("&v[0]: {:p}",&v[0]);
    //ora risposta è ptr: 0xbb8870, capacity:4,len: 1
    v.push(7); //nuovo dato
    //println!("ptr: {:p}, capacity:{},len: {}", v.as_ptr(),v.capacity(),v.len() );
    //ptr: 0xc62750, capacity:4,len: 2
    v.push(1);
    v.push(32);

    println!("ptr: {:p}, capacity:{},len: {}", v.as_ptr(),v.capacity(),v.len() );
    v.push(23);
    //possiamo compattare
    v.shrink_to_fit(); //libero un pezzo per darlo all's0
    println!("ptr: {:p}, capacity:{},len: {}", v.as_ptr(),v.capacity(),v.len() );
    println!("&v[0]: {:p}",&v[0]);
    v.pop();
    println!("ptr: {:p}, capacity:{},len: {}", v.as_ptr(),v.capacity(),v.len() );
    println!("&v[0]: {:p}",&v[0]); //non si è compattata, è cambiata solo la len

    //capacity duplica ogni volta
    //non si compatta mai: lo spazio in eccesso si butta sempre

    //voglio vedere come si comporta con gli interi: voglio vedere quando butta via interi
*/









}
