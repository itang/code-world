
trait Image {
    fn get_pixels<'a>(&'a self) -> &'a [u32];
}

struct Sprite<T> {
    bitmap: T,
    local_buf: ~[u32]
}

impl<'a, T: Image> Sprite<T> {
    fn new(input: T) -> Sprite<T> {
        Sprite { bitmap: input, local_buf: ~[]}
    }

    fn rotate_bitmap(&'a mut self, _degrees: uint) {
        let pixels = (*self).bitmap.get_pixels();
        (*self).local_buf.clear();
        (*self).local_buf.push_all(pixels);
        println!("{:?}", self.local_buf);
        //
    }
}

impl Image for ~[u32] {
    fn get_pixels<'a>(&'a self) -> &'a [u32]{
        self.as_slice()
    }
}

struct JPGImage {
    data : ~[u32]
}

impl Image for JPGImage{
    fn get_pixels<'a>(&'a self) -> &'a [u32]{
        self.data.as_slice()
    }
}

fn main() {
    let a = ~[1u32];
    let mut s1 = Sprite::new(a);
    let mut s2 = Sprite::new(~[2u32]);
    let mut s3 = Sprite::new(JPGImage{data: ~[1, 2]});
    s1.rotate_bitmap(10);
    s2.rotate_bitmap(10);
    s3.rotate_bitmap(10);
    println!("{:?}, {:?}", s1, s2);
    println!("{:?}", s3);
}