Esse projeto possui funções úteis para a manipulação de vetores e criação de imagens.
<a href="https://www.w3schools.com/html/">Veja meu artigo no linkedin</a> Nele, a lógica por trás desse código é descrita em maior detalhe.
## Manual do usuário:

#### O exemplo dentro de Fn main():

O código extrai os bytes de imagem chamada `minha_img.jpg`. Também obtém suas dimensões (largura `x` e altura `y`). 
Ele assume que essa imagem está no formato RGB. Em seguida, o código itera por todos os pixels da imagem, linha por linha e coluna por coluna. Para cada um, verifica se a sua posição (coordenadas `larg` e `alt`) se encontra na região central da imagem, especificamente dentro de um retângulo que exclui 1/4 das bordas superior, inferior, esquerda e direita. 
Se o pixel estiver dentro dessa zona, o código localiza onde os seus bytes correspondentes às cores (RGB) estão armazenados no vetor. Define os valores desses bytes para 255 (o que resulta na cor branca). Pixels fora dessa área central não são alterados.
Por último, o vetor de bytes modificado é utilizado para criar um novo arquivo de imagem JPG. Ele é chamado `vetor.jpg`, Utilizando as dimensões originais da imagem extraída.

Abaixo, veja uma descrição simplificada das funções utilizadas nesse exemplo:
#### **Função** get_bytes():
Extrai os dados de pixels (bytes) e as dimensões (x,y) de uma imagem. 
**Formatos Suportados:** (RGB u8) ou (RGBA u8) ( jpg / png ) 
**Parâmetros:**
- `nome` (string): Path ou nome da imagem.
- `force_size` (u32, booleano): Se `True`, redimensiona a imagem para essa largura definida, se `False` mantém a imagem na sua proporção original. 
-**Retorno:** Uma tupla `(vetor_u8_bytes, (largura, altura))`, onde `vetor_u8_bytes` é um vetor u8 contendo os bytes dos pixels e `(largura, altura)` são as dimensões dessa da imagem.

#### **Função** create():
A função `create` aceita:
- `nome`: Uma referência a string (`&str`) É o nome, também o path, para o arquivo.
- `vec_pixels`: Um vetor de bytes (`Vec<u8>`) contendo os dados sequenciais dos pixels.
- `x`: Um `Option<usize>` representa a largura desejada. `None` se não especificada.
- `y`: Um `Option<usize>` representa a altura desejada. `None` se não especificada.
- `pixel_len`: Uma tupla `(bool, bool)` para determinar os bytes por pixel ( .jpg / png / FF = Gray ).
Internamente:
1. Determina `num_pixel_len` (bytes por pixel) com base em `pixel_len`. (3 = jpg, 4 = png, 1 = gray)
2. Calcula `wx` e `hy` ( largura e altura finais ):
    - Se `x` e `y` são `Some`, usa os valores fornecidos.
    - Se ambos `x` e `y` são `None`, calcula a raiz quadrada da área total de pixels para `wx` e deriva `hy`. 
    - Se apenas `x` ou `y` for `Some`, usa o valor fornecido para calcular a dimensão faltante.
3. Valida se `vec_pixels.len()` é suficiente para `wx * hy * num_pixel_len`. Se for menor, causa um `panic`. 
4. Utiliza a biblioteca `image` para criar a imagem:
    - Faz um `match` em `num_pixel_len`:
        - `1`: Cria `GrayImage`.
        - `3`: Cria `RgbImage`.
        - `4`: Cria `RgbaImage`.
5. Salva a imagem criada com base no `nome` (Path e formato).

#### **Função** `get_linear_pos_from_cartesian`: 
Esta função transforma uma coordenada de pixel (x, y) em uma posição unidimensional dentro de um vetor (que representa um canvas). 
Você informa o tamanho total do canvas (largura e altura) e a coordenada (x, y) que deseja quer localizar. A função retorna o índice correspondente nesse vetor. É útil para acessar dados num canvas armazenado linearmente.

#### **Função** `get_multi_byte_pos_from_cartesian`:
**Recebe**: as dimensões da imagem (largura `wx`, altura `hy`), as coordenadas cartesianas do pixel desejado (`px`, `py`), e um indicador do formato do pixel (`pixel_type`).
**Retorna**: os índices (posições) dos bytes que compõem um pixel dentro de um vetor de bytes. 

O formato é determinado pela tupla `pixel_type`:
- `(true, _, _)` indica RGB (3 bytes/pixel).
- `(false, true, _)` indica RGBA (4 bytes/pixel)
- `(false, false, true)` indica Escala de Cinza (1 byte/pixel).
**Calcula** onde os índices `r`, `g`, `b`, `a` estão. 
**Retorna** uma tupla `(usize, usize, usize, usize)` com esses índices. 


