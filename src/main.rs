fn main() {
    
    let (mut vetor,(x,y)) 
        = 
    get_bytes("minha_img.jpg", (100,true)); 

//O sistema abaixo está configurado para 'jpg' ( R G B )

    for alt in 0..y{
        for larg in 0..x{
            if (alt > y/4 && alt < y - y/4 ) && (larg > x/4 && larg < x - x/4) {
                let (r,g,b,a)
                    = 
                get_multi_byte_pos_from_cartesian(
                    (x,y),
                    (larg,alt), 
                    (true,false,false)//RGB? | RGBA? | grayscale?
                );
                    vetor[r] = 255;
                    vetor[g] = 255;
                    vetor[b] = 255;
                 // vetor[a] = 255;
            }
        }
    }

    create("vetor.jpg",
     vetor,
     Some(x),
     Some(y), 
     (true,false) //RGB? | RGBA? | F F = grayscale
    );

}

pub fn get_bytes( //ler bytes da imagem alvo. receber vetor e proporção 
    mut nome:&str,
    force_size:(u32,bool)
)
-> (Vec<u8>, (usize, usize))
{

    use std::path::Path; use image::{open, Rgb, Rgba};

    if nome.contains("."){
        let nome_slice:Vec<&str> = nome.split(".").collect();
        nome = nome_slice[0];
    }
    
        let j = format!("{nome}.jpg");
        let p = format!("{nome}.png");
        
        let jpg = seek_file(j.as_str());
        let png = seek_file(p.as_str());

    if jpg {//resgate RGB
            let rawimage =  open(Path::new(&j)).unwrap();

            let dynamic_data:image::ImageBuffer<Rgb<u8>, Vec<u8>>;
                
            if force_size.1 { 
                dynamic_data = rawimage.thumbnail(force_size.0, force_size.0)
                .to_rgb8(); 
            }else{
                dynamic_data = rawimage.to_rgb8();
            };
 
         return (
                 dynamic_data.to_vec(), 
                 (dynamic_data.dimensions().0 as usize, dynamic_data.dimensions().1 as usize)
                );
 
    }else if png {//resgate rgbA
            let rawimage =  open(Path::new(&p)).unwrap();
            let dynamic_data:image::ImageBuffer<Rgba<u8>, Vec<u8>>;
                
            if force_size.1 { 
                dynamic_data = rawimage.thumbnail(force_size.0, force_size.0)
                .to_rgba8(); 
            }else{
                dynamic_data = rawimage.to_rgba8();
            };
 
         return (//"all_data" recebe isso
                 dynamic_data.to_vec(), 
                 (dynamic_data.dimensions().0 as usize, dynamic_data.dimensions().1 as usize)
                );
    }else { 
        panic!("ERRO FATAL: a imagem em 'get_bytes' não foi encontrada ou seu 'tipo' é invalido.");
    }; 

    fn seek_file(nome:&str)->bool{
        use std::fs::File;
        
        let state 
            = 
        if File::open(nome).is_err(){ false }
        else{ true };
        
        return state;
    } 

}


//---------------------------------------

pub fn create(// criar imagem por vetor
    nome:&str,
    vec_pixels:Vec<u8>,
    x:Option<usize>, 
    y:Option<usize>,
    pixel_len:(bool,bool)
){

    let mut wx:usize = 1;
    let mut hy:usize = 1;

    let num_pixel_len 
        = 
    if pixel_len.0 {3}
    else if pixel_len.1 {4}
    else{1};

    if x!=None && y !=None {
        wx=x.unwrap();
        hy=y.unwrap();
    }

    if x==None && y==None {
        let px_area = (vec_pixels.len()/num_pixel_len) as f32;
        let teorico_lad = px_area.sqrt() as usize;
     
            wx = teorico_lad; 
            hy = (vec_pixels.len()/num_pixel_len)/wx;    
    }

    if x==None && y!=None {
        hy=y.unwrap();
        let missing_x = (vec_pixels.len()/num_pixel_len)/y.unwrap();
        wx = missing_x;
    }

    if y==None && x!=None {
        wx=x.unwrap();
        let missing_y = (vec_pixels.len()/num_pixel_len)/x.unwrap();
        hy = missing_y;
    }

    use image::{RgbImage,RgbaImage,GrayImage};

    if vec_pixels.len() < (wx*hy)*num_pixel_len {
       panic!("-----> ERRO 'create': bytes faltantes ou 'width/height' incorretos")
    }else{
        match num_pixel_len {
            1 => {
                let img: GrayImage = GrayImage::from_vec(wx as u32, hy as u32,vec_pixels).expect("msg");
                img.save(nome).unwrap()
                }
            3 => { 
                let img: RgbImage = RgbImage::from_vec(wx as u32, hy as u32,vec_pixels).expect("Falha ao resgatar RGB");
                img.save(nome).unwrap()
                }
            4 => {
                let img: RgbaImage = RgbaImage::from_vec(wx as u32, hy as u32,vec_pixels).expect("msg");
                img.save(nome).unwrap()
                }
            _=>{println!("XXX ERRO -> 'create': Valor 'pixel_size' é inválido")} 
        }//match
}//else
   
}

//-------------------------------

pub fn get_linear_pos_from_cartesian( //converão de posição cartesiana para vetor
    canvas_wx_hy:(usize,usize),
    cx_cy_pos:(usize,usize),
)->usize{
    let mut pos = 0;

    let (wid,hei) = canvas_wx_hy;
    let (mut x,mut y) = cx_cy_pos;

        if (hei-1) < y {y=hei-1}//não passar limite 'y'
        if (wid-1) < x {x=wid-1}//não passar limite 'x'

        if y > 0 {pos=x + (wid*y) }
             else{ pos=x }
                
    return pos as usize;
}

//------------------------------

pub fn get_multi_byte_pos_from_cartesian( //resgatar trios/quartetos de byte
    targ_wx_hy:(usize,usize),//tamanho da imagem alvo
    targ_px_py:(usize,usize),//posição cartesiana desejada
    pixel_type:(bool,bool,bool)//tipo de pixel? (jpg?, png?, gray?)
)->(usize,usize,usize,usize){

    let (mut r,mut g,mut b,mut a) = (0,0,0,0);

    let (mut px,mut py) = targ_px_py;
    let (wx,hy) = targ_wx_hy;
    let (rgb,rgba,gray) = pixel_type;

    if px>wx-1{px = wx-1}; if py>hy-1{py = hy-1};

    if rgb {
       let rgb_px_start = px*3;
       if py > 0{
            r = rgb_px_start + ((wx)*3*py);
       }else{
            r = rgb_px_start
       }
       g = r+1;
       b = g+1;

    }else if rgba {
       let rgba_px_start = px*4;
       if py > 0{
            r = rgba_px_start + ((wx)*4*py);
       }else{
            r = rgba_px_start
       }
       g = r+1;
       b = g+1;
       a = b+1;

    }else if gray {
       let gray_px_start = px;
       if py > 0{
            r = gray_px_start + ((wx)*py);
       }else{
            r = gray_px_start
       }
    }

    return (r,g,b,a);
}