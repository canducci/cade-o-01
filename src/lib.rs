// Cadê o 01? — paródia de perseguição geográfica. Lógica toda aqui; o JS só desenha texto e botões.
use std::cell::RefCell;

struct City {
    name: &'static str,
    desc: &'static str,
    people: [&'static str; 3], // testemunhas locais
    clues: [&'static str; 4],
    vague: [&'static str; 4], // pistas sem nome de lugar, para os postos altos
}

const BSB: usize = 0;
const SP: usize = 1;
const RIO: usize = 2;
const BUE: usize = 3;
const GYE: usize = 4;
const MIA: usize = 5;

// lon/lat de cada cidade, mesma ordem de CITIES (e de CITY_LL no index.html)
const LL: [(f64, f64); 12] = [(-47.9, -15.8), (-46.6, -23.5), (-43.2, -22.9), (-58.4, -34.6), (-79.9, -2.2), (-80.2, 25.8),
    (-81.4, 28.5), (-77.0, 38.9), (35.2, 31.8), (12.5, 41.9), (-9.1, 38.7), (-16.9, 32.7)];

// horas de voo: 1h de embarque + 1h a cada 900 km (haversine)
fn flight(a: usize, b: usize) -> i32 {
    let ((l1, p1), (l2, p2)) = (LL[a], LL[b]);
    let (p1, p2, dl) = (p1.to_radians(), p2.to_radians(), (l2 - l1).to_radians());
    let h = ((p2 - p1) / 2.0).sin().powi(2) + p1.cos() * p2.cos() * (dl / 2.0).sin().powi(2);
    let km = 6371.0 * 2.0 * h.sqrt().asin();
    1 + (km / 900.0).round() as i32
}

const CITIES: &[City] = &[
    City { name: "Brasília", desc: "Capital federal. Ar seco, Esplanada dos Ministérios e muito foro privilegiado.",
        people: ["Lobista do Congresso", "Segurança do Planalto", "Vendedor de pamonha"], clues: [
        "Disse que ia voltar pra cidade em forma de avião desenhada por Lúcio Costa.",
        "Perguntou se o Lago Paranoá tinha vista pra alguma mansão de R$ 6 milhões.",
        "Queria chegar a tempo de uma sessão no Congresso Nacional.",
        "Reclamou da umidade de 12% do Planalto Central.",
    ], vague: [
        "Falou que ia pra uma cidade planejada no meio do cerrado.",
        "Disse que lá até a catedral parece uma coroa de concreto.",
        "Reclamou que lá não tem esquina, só tesourinha.",
        "Disse que ia pra onde o endereço parece senha: SQS, bloco, apartamento.",
    ]},
    City { name: "São Paulo", desc: "Maior cidade do país. Trânsito, garoa e banqueiros na Faria Lima.",
        people: ["Banqueiro da Faria Lima", "Pasteleiro da feira", "Motoboy"], clues: [
        "Falou que tinha reunião com banqueiros na Faria Lima.",
        "Perguntou onde ficava o MASP, na Avenida Paulista.",
        "Levou guarda-chuva por causa da garoa.",
        "Queria comer pastel de feira e ver um jogo no Morumbi.",
    ], vague: [
        "Queria provar a pizza que dizem ser melhor que a de Nápoles.",
        "Disse que ia pra maior cidade do hemisfério sul.",
        "Disse que ia pra onde a garoa é marca registrada.",
        "Queria ver o maior engarrafamento do país num feriado.",
    ]},
    City { name: "Rio de Janeiro", desc: "Cidade Maravilhosa. Cristo Redentor, praia e a velha ALERJ.",
        people: ["Vendedor de mate na praia", "Ex-servidor da ALERJ", "Surfista do Arpoador"], clues: [
        "Comentou saudade dos tempos de deputado na ALERJ.",
        "Queria ver o Cristo Redentor de perto.",
        "Falou de uma loja de chocolates num shopping da Barra da Tijuca.",
        "Ia assistir a um jogo no Maracanã.",
    ], vague: [
        "Disse que ia pra antiga capital, de frente pro mar.",
        "Queria subir um morro de bondinho pra ver a baía.",
        "Disse que ia passar o réveillon de branco numa praia famosa.",
        "Perguntou se o biscoito de polvilho ainda vende na areia.",
    ]},
    City { name: "Buenos Aires", desc: "Capital argentina. Tango, parrilla e Casa Rosada.",
        people: ["Dançarina de tango", "Parrillero de San Telmo", "Cambista do dólar blue"], clues: [
        "Trocou reais por pesos argentinos.",
        "Queria tirar foto no Obelisco da Avenida 9 de Julio.",
        "Perguntou o caminho para a Casa Rosada.",
        "Pediu doce de leite e uma aula de tango.",
    ], vague: [
        "Disse que ia pra onde o dólar tem cor.",
        "Perguntou se os vizinhos ainda falam de Maradona em cada esquina.",
        "Queria jantar um bife de chorizo bem tarde, depois das onze.",
        "Disse que ia pra onde chamam ônibus de 'colectivo'.",
    ]},
    City { name: "Guayaquil", desc: "Maior cidade do Equador, porto no Pacífico. Calor e iguanas no parque.",
        people: ["Vendedor de chapéu-panamá", "Guia do Malecón", "Atendente do aeroporto"], clues: [
        "Comemorou que lá usam dólar americano, mas se fala espanhol.",
        "Queria passear no Malecón 2000, à beira do rio Guayas.",
        "Perguntou se dava pra ver iguanas soltas num parque do centro.",
        "Disse que ia pra um país com nome de linha imaginária.",
    ], vague: [
        "Disse que ia pra um porto no Pacífico onde a moeda é o dólar.",
        "Comprou um chapéu que leva o nome de outro país.",
        "Disse que ia pra um país cortado ao meio pela linha do equador.",
        "Queria ver um parque cheio de iguanas no meio da cidade.",
    ]},
    City { name: "Miami", desc: "Flórida. Praia, Little Havana e brasileiros comprando eletrônicos.",
        people: ["Salva-vidas de South Beach", "Barista de Little Havana", "Brasileiro com seis malas"], clues: [
        "Comprou óculos escuros pra ir a South Beach.",
        "Quis tomar um cafecito em Little Havana.",
        "Disse que ia pra Flórida, mas não pra ver o Mickey.",
        "Perguntou quanto tempo de carro até Mar-a-Lago, em Palm Beach.",
    ], vague: [
        "Disse que ia pra onde se fala mais espanhol que inglês, mas nos EUA.",
        "Queria pegar sol numa praia de prédios art déco.",
        "Queria um sanduíche cubano e um cafezinho bem forte.",
        "Disse que ia pra onde os brasileiros voltam com seis malas de eletrônicos.",
    ]},
    City { name: "Orlando", desc: "Flórida central. Parques temáticos e filas intermináveis.",
        people: ["Pessoa fantasiada de mascote", "Vendedor de ingressos", "Brasileira na fila da montanha-russa"], clues: [
        "Queria tirar foto com um camundongo famoso.",
        "Perguntou sobre ingressos para parques temáticos.",
        "Ia fazer compras nos outlets da Flórida, longe da praia.",
        "Disse que ia pra capital mundial das montanhas-russas.",
    ], vague: [
        "Disse que ia pra uma cidade sem praia, mas cheia de castelos.",
        "Reclamou do calor do centro da Flórida.",
        "Comprou orelhas de rato pra usar numa fila de três horas.",
        "Disse que ia pra onde todo brasileiro de 15 anos quer viajar.",
    ]},
    City { name: "Washington", desc: "Capital dos EUA. Casa Branca, Capitólio e lobistas.",
        people: ["Lobista da K Street", "Guarda do Capitólio", "Estagiário do Depto. de Estado"], clues: [
        "Tinha reunião marcada no Departamento de Estado.",
        "Queria ver a Casa Branca pessoalmente.",
        "Perguntou onde fica o Capitólio.",
        "Disse que ia pra uma cidade com um obelisco gigante e muitos lobistas.",
    ], vague: [
        "Disse que ia pra uma capital com nome de presidente.",
        "Queria ver de perto o Congresso que invadiram em 2021, só que o de lá.",
        "Disse que ia pra onde fica a casa mais vigiada do mundo.",
        "Queria visitar museus de graça numa capital estrangeira.",
    ]},
    City { name: "Jerusalém", desc: "Cidade sagrada para três religiões, em Israel.",
        people: ["Guia da Cidade Velha", "Vendedor de falafel", "Peregrino"], clues: [
        "Trocou dinheiro por shekels.",
        "Queria rezar no Muro das Lamentações.",
        "Disse que ia a um país com a Estrela de Davi na bandeira.",
        "Estava treinando o 'shalom'.",
    ], vague: [
        "Disse que ia rezar numa cidade sagrada para três religiões.",
        "Queria levar uma garrafa de água do rio Jordão.",
        "Perguntou se lá o transporte para no sábado.",
        "Disse que ia pra uma cidade murada, cheia de peregrinos.",
    ]},
    City { name: "Roma", desc: "Capital da Itália. Coliseu, Vaticano e burocracia consular.",
        people: ["Atendente da cidadania italiana", "Gladiador de foto", "Guarda suíço"], clues: [
        "Mostrou orgulhoso o passaporte italiano.",
        "Queria ver o Coliseu.",
        "Perguntou se o Papa recebia visitas.",
        "Pagou em euro e pediu um espresso.",
    ], vague: [
        "Disse que todos os caminhos levam pra lá.",
        "Queria jogar uma moeda numa fonte pra garantir a volta.",
        "Disse que ia pra onde fica o menor país do mundo, dentro de uma cidade.",
        "Queria ver uma arena onde gladiadores lutavam.",
    ]},
    City { name: "Lisboa", desc: "Capital de Portugal. Pastel de nata, elétrico 28 e muitos brasileiros.",
        people: ["Motorneiro do elétrico 28", "Fadista de Alfama", "Pasteleiro de Belém"], clues: [
        "Queria comer pastel de Belém.",
        "Perguntou como pegar o elétrico 28.",
        "Disse que ia pra onde falam português 'com sotaque'.",
        "Ia ouvir fado em Alfama.",
    ], vague: [
        "Disse que ia pra terra do bacalhau, mas na capital.",
        "Queria ver a ponte vermelha sobre o Tejo.",
        "Disse que ia pra uma capital de bondinhos amarelos e ladeiras.",
        "Queria ouvir uma canção triste numa taberna antiga.",
    ]},
    City { name: "Funchal", desc: "Ilha da Madeira, Portugal. Vinho, levadas e empresas offshore.",
        people: ["Contador de offshore", "Guia das levadas", "Produtor de vinho Madeira"], clues: [
        "Disse que ia pra uma ilha portuguesa no meio do Atlântico.",
        "Queria provar vinho Madeira.",
        "Perguntou pela terra natal do Cristiano Ronaldo.",
        "Comentou de uma holding registrada numa ilha portuguesa.",
    ], vague: [
        "Disse que ia pra uma ilha famosa pelo réveillon com fogos.",
        "Queria descer a ladeira num carrinho de cesto.",
        "Disse que ia pra uma ilha onde nasceu um craque de futebol.",
        "Queria ver bananeiras na encosta de um vulcão.",
    ]},
];

struct Case {
    title: &'static str,
    brief: &'static str,
    start: usize,
    fixed: &'static [usize],
    dossie: &'static str,
    // (veículo e data, URL da reportagem)
    fontes: &'static [(&'static str, &'static str)],
}

const CASES: &[Case] = &[
    Case { title: "Rachadinha na ALERJ", start: RIO, fixed: &[],
        brief: "Segundo o MP-RJ, pedaços dos salários de assessores da ALERJ tomaram rumo ignorado. O 01 também tomou rumo ignorado. Coincidência?",
        dossie: "RACHADINHA NA ALERJ\n\
Alegação: o MP-RJ denunciou Flávio em nov/2020 por peculato, lavagem de dinheiro e organização criminosa, apontando um esquema de recolhimento de salários de assessores da ALERJ operado por Fabrício Queiroz (R$ 6,1 mi, segundo o MP).\n\
Status: denúncia rejeitada pelo TJ-RJ em mai/2022, depois que STJ e STF anularam provas e o próprio MP pediu a anulação. A decisão não impede nova investigação com provas válidas.\n\
Defesa: nega ter cometido crimes e diz que o caso está encerrado.",
        fontes: &[
            ("Brasil de Fato, 03/10/2026", "https://www.brasildefato.com.br/2026/10/03/flavio-bolsonaro-chega-ao-primeiro-turno-investigado-pela-pf-com-relacoes-com-vorcaro-expostas-e-vice-suspeito-de-estupro/"),
            ("InfoMoney, 17/05/2022", "https://www.infomoney.com.br/politica/tj-do-rio-arquiva-denuncia-do-mp-contra-flavio-bolsonaro-em-caso-das-rachadinhas/"),
            ("InfoMoney, 27/02/2025", "https://www.infomoney.com.br/politica/gilmar-nega-recursos-e-mantem-arquivado-caso-das-rachadinhas-de-flavio-bolsonaro/"),
            ("Poder360, 04/02/2021 (valor de R$ 6,1 mi)", "https://www.poder360.com.br/brasil/flavio-bolsonaro-entrega-loja-de-chocolates-investigada-pelo-mp/")] },
    Case { title: "Bombons Recheados", start: RIO, fixed: &[],
        brief: "Uma franquia de chocolates num shopping da Barra virou assunto do MP. O dono sumiu antes da Páscoa.",
        dossie: "FRANQUIA DE CHOCOLATES (KOPENHAGEN)\n\
Alegação: o MP-RJ apurou se a franquia de Flávio e do sócio Alexandre Santini serviu para lavar dinheiro do suposto esquema da ALERJ: ao menos R$ 2,1 mi, sendo R$ 500 mil na compra da loja e R$ 1,6 mi movimentados na conta da empresa.\n\
Status: linha de investigação do caso da rachadinha, cuja denúncia foi rejeitada em 2022. A loja foi entregue e fechada em 2021.\n\
Defesa: nega. Em 2019: \"Se eu quisesse lavar dinheiro iria abrir uma franquia, que tem um controle externo da franquiadora e auditoria?\"",
        fontes: &[
            ("O Povo, 20/08/2020", "https://www.opovo.com.br/noticias/politica/2020/08/20/investigacao-apura-se-loja-de-flavio-bolsonaro--lavou--r--2-1-milhoes.html"),
            ("Poder360, 04/02/2021", "https://www.poder360.com.br/brasil/flavio-bolsonaro-entrega-loja-de-chocolates-investigada-pelo-mp/"),
            ("Folha de Pernambuco, 21/10/2026", "https://www.folhape.com.br/politica/de-rachadinha-a-dark-horse-flavio-bolsonaro-e-alvo-de/519718/"),
            ("Correio, 19/12/2019 (frase da defesa)", "https://www.correio24horas.com.br/brasil/flavio-bolsonaro-nega-rachadinhas-com-queiroz-iria-abrir-franquia-1219")] },
    Case { title: "Imóveis no Dinheiro Vivo", start: RIO, fixed: &[],
        brief: "Corretores de Copacabana relatam pagamentos em espécie. O cliente pagou à vista e saiu à francesa.",
        dossie: "IMÓVEIS EM DINHEIRO VIVO\n\
Alegação: o MP-RJ apontou depósito de R$ 638,4 mil em espécie na conta de um corretor e \"ganhos artificialmente produzidos\" de R$ 800 mil com dois apartamentos em Copacabana. Um levantamento do UOL identificou 16 imóveis comprados ao menos em parte com dinheiro vivo, e a movimentação em espécie atribuída a Flávio passaria de R$ 3 mi.\n\
Status: parte do caso da rachadinha (denúncia rejeitada em 2022). Transação em espécie não é crime por si só.\n\
Defesa: nega irregularidades.",
        fontes: &[
            ("InfoMoney, 19/12/2019", "https://www.infomoney.com.br/politica/promotores-investigam-se-flavio-bolsonaro-lavou-r-638-mil-de-rachadinha-com-imoveis/"),
            ("Brasil de Fato, 03/10/2026", "https://www.brasildefato.com.br/2026/10/03/flavio-bolsonaro-chega-ao-primeiro-turno-investigado-pela-pf-com-relacoes-com-vorcaro-expostas-e-vice-suspeito-de-estupro/")] },
    Case { title: "A Mansão do Lago", start: BSB, fixed: &[],
        brief: "Uma mansão de R$ 6 milhões em Brasília está com as luzes apagadas. O morador saiu pra comprar pão. Em outro continente.",
        dossie: "MANSÃO EM BRASÍLIA\n\
Fato: em 2021 Flávio comprou uma casa por R$ 5,97 mi em Brasília, com R$ 3,1 mi financiados pelo BRB, banco público do DF. O MP-DF abriu procedimento para apurar se o empréstimo saiu fora das condições normais do banco.\n\
Status: nas fontes consultadas não há acusação criminal sobre a compra.\n\
Defesa: nega irregularidades na compra.",
        fontes: &[
            ("Poder360, 02/03/2021", "https://www.poder360.com.br/congresso/flavio-bolsonaro-nega-irregularidades-em-compra-de-mansao-por-r-6-milhoes/"),
            ("Poder360, 06/04/2021", "https://www.poder360.com.br/congresso/mp-investiga-emprestimo-do-brb-para-flavio-bolsonaro-comprar-mansao/")] },
    Case { title: "Dark Horse", start: SP, fixed: &[],
        brief: "A PF segue o dinheiro de um banqueiro para uma cinebiografia. O produtor-executivo informal sumiu antes dos créditos.",
        dossie: "DARK HORSE / VORCARO\n\
Alegação: a PF investiga pagamentos do ex-banqueiro Daniel Vorcaro (Banco Master) ao filme 'Dark Horse', sobre Jair Bolsonaro. O The Intercept Brasil publicou áudios de Flávio pedindo recursos a Vorcaro para o filme. Suspeitas: lavagem, corrupção e evasão de divisas.\n\
Status: Flávio foi incluído no inquérito do STF em jul/2026 (min. André Mendonça); em andamento.\n\
Defesa: nega vantagem indevida; diz que foi \"patrocínio privado para um filme privado\".",
        fontes: &[
            ("Agência Brasil, 11/09/2026", "https://agenciabrasil.ebc.com.br/justica/noticia/2026-09/mendonca-incluiu-flavio-bolsonaro-como-investigado-no-caso-dark-horse"),
            ("Brasil de Fato, 03/10/2026", "https://www.brasildefato.com.br/2026/10/03/flavio-bolsonaro-chega-ao-primeiro-turno-investigado-pela-pf-com-relacoes-com-vorcaro-expostas-e-vice-suspeito-de-estupro/"),
            ("Agência Brasil, 27/10/2026", "https://agenciabrasil.ebc.com.br/politica/noticia/2026-10/policia-federal-cancela-passaportes-de-flavio-bolsonaro")] },
    Case { title: "Aposta Perdida", start: BSB, fixed: &[],
        brief: "Um inquérito sobre bets cita o nome do senador. Ele apostou que não ia ficar pra ver o resultado.",
        dossie: "BETS / WILLER TOMAZ\n\
Alegação: Flávio e o advogado Willer Tomaz são investigados no STF por suposta lavagem de dinheiro em negócios com bets. Segundo a PF, o senador seria beneficiário de recursos da Alcazar Holdings, sediada na Ilha da Madeira. Em 2025, uma secretária de Tomaz pediu ao Santander que tirasse o nome de Flávio da conta da empresa, onde ele aparecia como \"usuário master\".\n\
Status: inquérito no STF sob o min. Alexandre de Moraes, em andamento.\n\
Defesa: o senador nega irregularidades.",
        fontes: &[
            ("Agência Brasil, 26/10/2026", "https://agenciabrasil.ebc.com.br/politica/noticia/2026-10/moraes-manda-flavio-entregar-passaporte-e-o-proibe-de-deixar-o-pais"),
            ("Agência Brasil, 27/10/2026", "https://agenciabrasil.ebc.com.br/politica/noticia/2026-10/policia-federal-cancela-passaportes-de-flavio-bolsonaro"),
            ("Revista Fórum, 23/10/2026", "https://revistaforum.com.br/politica/flavio-bolsonaro-e-willer-tomaz-entenda/"),
            ("Jornal do Brasil, 14/10/2026", "https://www.jb.com.br/brasil/policia/2026/10/1061225-pf-cita-flavio-bolsonaro-como-usuario-master-de-conta-de-empresa-de-advogado.html")] },
    Case { title: "Emendas Misteriosas", start: BSB, fixed: &[],
        brief: "Emendas parlamentares foram parar em institutos 'inexistentes de fato'. O padrinho delas também ficou inexistente de fato.",
        dossie: "EMENDAS PARLAMENTARES\n\
Alegação: reportagens apontam emendas indicadas por Flávio que pagaram R$ 666 mil a um instituto que a Receita classificou como \"inexistente de fato\" (IMP, em Mesquita-RJ), cerca de R$ 200 mil a uma ONG ligada aos irmãos Brazão (IFOP) e empresas investigadas pela PF por fraude em licitação.\n\
Status: apurações da PF, do TCE-RJ e da CGU. Não há acusação criminal contra ele por esses pontos.\n\
Defesa: diz que o gabinete indica emendas mas não escolhe as empresas contratadas.",
        fontes: &[
            ("Brasil de Fato, 01/10/2026", "https://www.brasildefato.com.br/2026/10/01/emendas-de-flavio-bolsonaro-chegaram-a-empresa-acusada-de-cartel-ong-inexistente-e-instituto-indicado-por-miliciano/"),
            ("Brasil de Fato, 03/10/2026", "https://www.brasildefato.com.br/2026/10/03/flavio-bolsonaro-chega-ao-primeiro-turno-investigado-pela-pf-com-relacoes-com-vorcaro-expostas-e-vice-suspeito-de-estupro/")] },
    Case { title: "O Passaporte Que Não Voltou", start: BSB, fixed: &[BSB, BUE, GYE, MIA],
        brief: "O STF mandou entregar os passaportes em 24 horas. Passaram-se 24 horas. Os passaportes não. O senador também não.",
        dossie: "PASSAPORTES / SAÍDA DO PAÍS\n\
Fatos: em 25/10/2026 o min. Alexandre de Moraes proibiu Flávio de deixar o país e mandou entregar os passaportes em 24h; a intimação foi em 26/10. O prazo venceu sem entrega e a PF cancelou os passaportes. Segundo reportagens, na noite de 27/10 ele cruzou para a Argentina, fez escala em Guayaquil (Equador) e chegou a Miami em 28/10. Ele tem passaporte italiano emitido em 2023.\n\
Status: fatos em desenvolvimento (28/10/2026). Os dois inquéritos no STF continuam.",
        fontes: &[
            ("Agência Brasil, 26/10/2026", "https://agenciabrasil.ebc.com.br/politica/noticia/2026-10/moraes-manda-flavio-entregar-passaporte-e-o-proibe-de-deixar-o-pais"),
            ("Gazeta do Povo, 26/10/2026 (prazo de 24h)", "https://www.gazetadopovo.com.br/republica/flavio-bolsonaro-tem-passaportes-apreendidos-e-esta-proibido-de-deixar-o-pais/"),
            ("Agência Brasil, 27/10/2026", "https://agenciabrasil.ebc.com.br/politica/noticia/2026-10/policia-federal-cancela-passaportes-de-flavio-bolsonaro"),
            ("Nexo, 27/10/2026", "https://www.nexojornal.com.br/expresso/2026/10/27/flavio-bolsonaro-fugiu-argentina-pf-passaporte"),
            ("Brasil de Fato, 28/10/2026", "https://www.brasildefato.com.br/2026/10/28/flavio-bolsonaro-fugiu-pela-argentina-na-noite-de-terca-27-e-fez-escala-no-equador-antes-de-seguir-para-os-eua/")] },
];

const WITNESSES: [&str; 8] = [
    "Taxista", "Garçom", "Assessor que nunca foi ao gabinete", "Vendedora de bombons",
    "Corretor de imóveis", "Influencer de viagens", "Piloto de jatinho", "Vizinho fofoqueiro",
];
// índices 0..8 são genéricos; depois vêm os 3 locais de cada cidade, na ordem de CITIES
fn witness_name(i: usize) -> &'static str {
    if i < WITNESSES.len() { WITNESSES[i] } else { CITIES[(i - WITNESSES.len()) / 3].people[(i - WITNESSES.len()) % 3] }
}

// fala de abertura de cada testemunha local, mesma ordem de people
const HELLO: [&str; 36] = [
    "Meu cliente não comenta, mas eu sim.", "Circulando... tá bom, rapidinho.", "Olha a pamonha, pamonha fresquinha!",
    "Tempo é dinheiro, e o seu está acabando.", "Vai um de queijo enquanto conversa?", "Fala rápido que tenho entrega.",
    "Mate, limão, biscoito Globo!", "Trabalhei lá, mas nunca vi o gabinete por dentro.", "Altas ondas, brother.",
    "Un tango y te cuento.", "¿Jugoso o a punto? Bueno, escuchá.", "Cambio, cambio! Pago bem no blue.",
    "É panamá, mas é do Equador, viu?", "Bem-vindo ao Malecón 2000!", "Passaporte, por favor... ah, esse aí.",
    "Sem correr na areia!", "Un cafecito antes?", "Só vim comprar umas coisinhas.",
    "Shhh, mascote não fala... mas vou abrir exceção.", "Passe anual com desconto, quer?", "Tô há três horas nessa fila!",
    "Isso vai te custar, mas tudo bem.", "Mantenha distância da escadaria.", "Não sou autorizado a falar, mas...",
    "Siga meu guarda-chuva amarelo!", "Com tahine ou sem?", "Vim a pé desde a Galileia.",
    "Senha 4.812, aguarde... ah, é você.", "Foto comigo, cinco euros.", "Não posso sair da posição, então fala daí.",
    "Segure-se, que o 28 vai a abanar!", "Ai, que saudade...", "Pastéis saídos do forno, com canela!",
    "Discrição é o nosso produto.", "Cuidado com a beira da levada.", "Prove este Madeira de 1999.",
];
// "Fala. " para locais, vazio para genéricos
fn hello(i: usize) -> String {
    if i < WITNESSES.len() { String::new() } else { format!("{} ", HELLO[i - WITNESSES.len()]) }
}

// países por cidade, mesma ordem de CITIES
const COUNTRY: [usize; 12] = [0, 0, 0, 1, 2, 3, 3, 3, 4, 5, 6, 6];
// chance (%) de uma testemunha ser apoiadora do fugitivo, por posto: Estagiário nunca
const SUPPORT_PCT: [usize; 5] = [0, 10, 20, 30, 40];
const SUPPORT: [&str; 8] = [
    "Isso é perseguição política, e eu não falo com quem persegue!",
    "Eu apoio o senador de todo o coração. Não vi nada, não sei de nada.",
    "Perseguição! Fugiu coisa nenhuma, foi viajar com a família.",
    "E o Lula, hein? Quero ver investigarem ele também!",
    "E o PT, hein? Isso aí ninguém investiga, né?",
    "E o Lula, hein? Com o que ele fez, vocês vêm atrás do 01?",
    "E o PT, hein? Aí o inquérito nunca anda!",
    "Ele é um perseguido político. Pergunta pro Lula onde estão os passaportes dele!",
];

const NOTHING: [&str; 4] = [
    "Senador? Aqui? Nunca vi.",
    "Não vi ninguém de terno fugindo por aqui, não.",
    "O único político que passou aqui foi numa propaganda eleitoral.",
    "Hum... não. Tem certeza que é essa cidade?",
];
const PROVAS: [&str; 6] = [
    "um print de WhatsApp", "um comprovante de depósito em espécie", "uma nota fiscal esquisita",
    "um recibo de bombons", "um áudio vazado", "uma planilha misteriosa",
];
// fala de fim (vai pro balão), inspirada em declarações públicas, uma lista por resultado
const END_WIN: [&str; 6] = [
    "01: Isso é justiça? Prender senador por causa disso? Vou recorrer até o fim!",
    "Pai do 01: Mexeram com meu filho só pra me atingir. Já vi esse filme.",
    "03: Decisão arbitrária e sem efeito prático. Vou denunciar isso lá fora!",
    "Defesa: Isso é uma santa inquisição, toda baseada em premissas falsas!",
    "Aliado no Senado: É um Torquemada moderno! Usa o inquérito para intimidar.",
    "01: Moraes, faça um bem pra nação e renuncie. E devolve meu passaporte.",
];
const END_ESCAPE: [&str; 6] = [
    "01: Tchau, querido! Sem mandado, isso aqui é só um passeio de jatinho.",
    "03: Eu avisei: se apertar, sai do país. Aqui nos EUA sempre cabe mais um.",
    "01: Sair do país nunca foi opção. Isso aqui é só uma escala técnica.",
    "Pai do 01: Ninguém aqui foge. Ele só viajou mais rápido que o oficial de justiça.",
    "Defesa: Sem mandado não há caso. E o caso, como sempre dissemos, está enterrado.",
    "Aliado no Senado: Recebo com tranquilidade. Quem tem que explicar é o agente sem mandado.",
];
const END_TIME: [&str; 6] = [
    "01: Nunca respondi criminalmente por isso. Toda essa espuma já baixou.",
    "Pai do 01: Eu sempre disse: se errou e for provado, paga. Provar, ninguém provou.",
    "Defesa: Nulidades reconhecidas, prazo vencido. A defesa sempre confiou na Justiça.",
    "03: Sempre disse que era perseguição. Agora até o relógio concorda comigo.",
    "Aliado no Senado: Eu tinha certeza: a investigação mostraria que não há ato ilícito nenhum.",
    "01: Pode revirar do avesso. Ou, mais prático, deixar quietinho na gaveta.",
];
const END_ARCHIVE: [&str; 6] = [
    "01: Conhecereis a verdade, e a verdade vos libertará. Glória a Deus!",
    "01: Anos de perseguição, vazamentos e covardias, e nada contra mim. Justiça feita!",
    "Defesa: Como a defesa sempre disse: o caso está enterrado. Descanse em paz.",
    "Pai do 01: Sempre acreditei na inocência do meu filho. Usaram ele pra me atingir.",
    "03: Acabou a perseguição contra meu irmão! Agora só falta a do resto da família.",
    "Aliado no Senado: Recebi o arquivamento com a mesma tranquilidade com que recebi o inquérito.",
];

// carreira: casos resolvidos para chegar ao posto e folga sobre a rota sem erro
struct Posto { need: u32, name: &'static str, abbr: &'static str, slack: i32 }
const RANKS: [Posto; 5] = [
    Posto { need: 0, name: "Estagiário", abbr: "Estag.", slack: 40 },
    Posto { need: 1, name: "Agente", abbr: "Ag.", slack: 34 },
    Posto { need: 3, name: "Agente especial", abbr: "Ag. esp.", slack: 28 },
    Posto { need: 6, name: "Delegado", abbr: "Del.", slack: 22 },
    Posto { need: 10, name: "Delegado-chefe", abbr: "Del.-chefe", slack: 16 },
];
fn rank(solved: u32) -> usize { RANKS.iter().rposition(|r| solved >= r.need).unwrap() }
// a partir de Delegado: só pistas vagas e destinos errados perto do certo
const SHARP: usize = 3;
const H_ASK: i32 = 2;
// depois de cada viagem, 1 em EVENT_ODDS chances de um evento (atraso, habeas corpus, vazamento)
const EVENT_ODDS: usize = 5;
const H_DELAY: i32 = 4;
const H_WARRANT: i32 = 3;
const NEED: u32 = 2;
// Modo difícil: 1 em ARCHIVE_ODDS viagens termina em arquivamento.
const ARCHIVE_ODDS: usize = 10;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Screen { Title, DossieList, Dossie(usize), Brief, City, Travel, End }

pub struct Game {
    rng: u64,
    screen: Screen,
    case: usize,
    route: Vec<usize>,
    step: usize,
    here: usize,
    hours: i32,
    provas: u32,
    prova_slot: usize,
    prova_taken: bool,
    last_ask: Option<usize>,
    mandado: bool,
    hard: bool,
    solved: u32,
    witnesses: [usize; 3],
    supporter: Option<usize>,
    support_seen: Vec<usize>, // países que já tiveram apoiador neste caso
    dests: Vec<usize>,
    msg: String,
    end: String,
}

impl Game {
    pub fn new(seed: u32, solved: u32) -> Game {
        Game {
            rng: seed as u64 | 1, screen: Screen::Title, case: 0, route: vec![], step: 0, here: 0,
            hours: 0, provas: 0, prova_slot: 0, prova_taken: false, last_ask: None, mandado: false, hard: false, solved,
            witnesses: [0, 1, 2], supporter: None, support_seen: vec![], dests: vec![], msg: String::new(), end: String::new(),
        }
    }

    fn rand(&mut self, n: usize) -> usize {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng % n as u64) as usize
    }

    fn posto(&self) -> &'static Posto { &RANKS[rank(self.solved)] }
    fn sharp(&self) -> bool { rank(self.solved) >= SHARP }
    // prazo = rota sem erro (voos + 3 interrogatórios por cidade + mandado) + folga do posto
    fn deadline(&self) -> i32 {
        let flights: i32 = self.route.windows(2).map(|w| flight(w[0], w[1])).sum();
        flights + self.route.len() as i32 * 3 * H_ASK + H_WARRANT + self.posto().slack
    }

    fn last(&self) -> usize { self.route.len() - 1 }
    fn on_trail(&self) -> bool { self.here == self.route[self.step] }

    fn new_case(&mut self, hard: bool) {
        self.hard = hard;
        self.case = self.rand(CASES.len());
        let c = &CASES[self.case];
        self.route = if c.fixed.is_empty() {
            let mut r = vec![c.start];
            while r.len() < 5 {
                let x = self.rand(CITIES.len());
                if !r.contains(&x) { r.push(x); }
            }
            r
        } else {
            c.fixed.to_vec()
        };
        self.step = 0;
        self.hours = self.deadline();
        self.provas = 0;
        self.mandado = false;
        self.support_seen.clear();
        self.msg.clear();
        self.arrive(self.route[0]);
        self.screen = Screen::Brief;
    }

    fn arrive(&mut self, city: usize) {
        self.here = city;
        self.prova_taken = false;
        self.last_ask = None;
        // duas testemunhas locais + uma genérica, em posições sorteadas
        let a = self.rand(3);
        let b = (a + 1 + self.rand(2)) % 3;
        let local = |k| WITNESSES.len() + city * 3 + k;
        let mut w = [local(a), local(b), self.rand(WITNESSES.len())];
        w.rotate_left(self.rand(3));
        self.witnesses = w;
        // no máximo 1 apoiador por país (por caso), com chance crescente conforme o posto
        self.supporter = None;
        let country = COUNTRY[city];
        if !self.support_seen.contains(&country) && self.rand(100) < SUPPORT_PCT[rank(self.solved)] {
            self.supporter = Some(self.rand(3));
            self.support_seen.push(country);
        }
        self.prova_slot = match self.supporter {
            Some(slot) => (slot + 1 + self.rand(2)) % 3,
            None => self.rand(3),
        };
    }

    fn finish(&mut self, text: String) {
        let pool = if text.contains("🏆") { END_WIN } else if text.contains("ARQUIVADO") { END_ARCHIVE }
            else if text.contains("⏰") { END_TIME } else { END_ESCAPE };
        let (who, line) = pool[self.rand(pool.len())].split_once(": ").unwrap();
        self.end = format!("{}\n\" {}: \"{}\"", text, who, line);
        self.screen = Screen::End;
    }

    fn check_time(&mut self) {
        if self.hours <= 0 && self.screen != Screen::End {
            self.finish("⏰ O prazo acabou. O caso foi parar numa gaveta, ao lado de outros tantos.\nO 01 manda lembranças de algum lugar com wi-fi.".into());
        }
    }

    fn ask(&mut self, slot: usize) {
        self.last_ask = Some(slot);
        self.hours -= H_ASK;
        let who = witness_name(self.witnesses[slot]);
        let hi = hello(self.witnesses[slot]);
        if self.supporter == Some(slot) {
            self.msg = format!("\" {}: \"{}{}\"", who, hi, SUPPORT[self.rand(SUPPORT.len())]);
        } else if !self.on_trail() {
            self.msg = format!("\" {}: \"{}{}\"", who, hi, NOTHING[self.rand(NOTHING.len())]);
        } else if self.step == self.last() {
            let city = CITIES[self.here].name;
            if self.mandado {
                let before = rank(self.solved);
                self.solved = self.solved.saturating_add(if self.hard { 2 } else { 1 });
                let now = rank(self.solved);
                let promo = if now > before { format!("\n\n⭐ Promovido a {}!", RANKS[now].name) } else { String::new() };
                self.finish(format!("🚔 {} aponta uma esquina de {}. Lá está ele, de boné e óculos escuros.\nVocê mostra o mandado do STF, e ele pede pra falar com o advogado.\n\n🏆 CASO ENCERRADO (por enquanto — ainda cabe recurso).{}", who, city, promo));
            } else {
                self.finish(format!("🛩️ {} aponta uma esquina de {}. Lá está ele!\nMas você não tem mandado. Ele acena, sorri e embarca num jatinho.\n\n❌ Sem mandado, sem prisão. Junte provas e peça ao STF antes do confronto.", who, city));
            }
        } else {
            let next = self.route[self.step + 1];
            let c = &CITIES[next];
            let clue = if self.sharp() { c.vague[self.rand(4)] } else { c.clues[self.rand(4)] };
            self.msg = format!("\" {}: \"{}{}\"", who, hi, clue);
            if slot == self.prova_slot && !self.prova_taken {
                self.prova_taken = true;
                self.provas += 1;
                let p = PROVAS[self.rand(PROVAS.len())];
                self.msg += &format!("\n+ 📎 De quebra, entregou {}. (+1 prova)", p);
            }
        }
        self.check_time();
    }

    fn open_travel(&mut self) {
        let correct = if !self.on_trail() { Some(self.route[self.step]) }
            else if self.step < self.last() { Some(self.route[self.step + 1]) }
            else { None };
        let mut d: Vec<usize> = correct.into_iter().collect();
        // nos postos altos, os errados saem das 5 cidades mais perto do certo (Miami/Orlando, Lisboa/Funchal)
        let mut near: Vec<usize> = (0..CITIES.len()).filter(|&x| x != self.here && Some(x) != correct).collect();
        if let (true, Some(c)) = (self.sharp(), correct) {
            near.sort_by_key(|&x| flight(c, x));
            near.truncate(5);
        }
        while d.len() < 4 {
            let x = near.swap_remove(self.rand(near.len()));
            d.push(x);
        }
        for i in (1..d.len()).rev() {
            let j = self.rand(i + 1);
            d.swap(i, j);
        }
        self.dests = d;
        self.screen = Screen::Travel;
    }

    fn travel(&mut self, dest: usize) {
        self.hours -= flight(self.here, dest);
        if self.hard && self.rand(ARCHIVE_ODDS) == 0 {
            self.finish("📁 O relator, terrivelmente evangélico, pediu vista, orou sobre o processo e arquivou o caso.\nO 01 agradece a Deus nas redes.\n\n❌ CASO ARQUIVADO.".into());
            return;
        }
        if self.on_trail() && self.step < self.last() && dest == self.route[self.step + 1] {
            self.step += 1;
        }
        self.arrive(dest);
        self.msg = if self.on_trail() && self.step == self.last() {
            "👀 Burburinho no aeroporto: alguém viu um senador de boné por aqui...".into()
        } else {
            "✈️ Você desembarca e procura testemunhas.".into()
        };
        if self.rand(EVENT_ODDS) == 0 { self.event(); }
        self.screen = Screen::City;
        self.check_time();
    }

    fn event(&mut self) {
        // habeas corpus só faz sentido com mandado na mão; sem ele, sorteia entre os outros dois
        let e = if self.mandado { self.rand(3) } else { self.rand(2) * 2 };
        self.msg += match e {
            0 => { self.hours -= H_DELAY; "\n- 🕓 Voo atrasado: a companhia aérea pede desculpas pelas 4h perdidas." }
            1 => { self.mandado = false; "\n- ⚖️ Habeas corpus surpresa: o mandado caiu. Peça outro ao STF." }
            _ => { self.provas += 1; "\n+ 📰 Vazamento à imprensa: uma reportagem entrega o que faltava. (+1 prova)" }
        };
    }

    fn warrant(&mut self) {
        self.hours -= H_WARRANT;
        if self.provas < NEED {
            self.msg = format!("⚖️ STF: \"Com {} prova(s)? Volte quando tiver pelo menos {}.\"", self.provas, NEED);
        } else {
            // A piada: cada prova pode ser anulada no caminho.
            let mut annulled = 0;
            for _ in 0..self.provas {
                if self.rand(if self.hard { 2 } else { 3 }) == 0 { annulled += 1; }
            }
            self.provas -= annulled;
            self.msg = if annulled > 0 { format!("- 🗑️ {} prova(s) anulada(s) por questões processuais.\n", annulled) } else { String::new() };
            if self.provas >= NEED {
                self.mandado = true;
                self.msg += "✅ Mandado expedido!";
            } else {
                self.msg += "❌ Mandado negado. Junte mais provas.";
            }
        }
        self.check_time();
    }

    fn options(&self) -> Vec<String> {
        match self.screen {
            Screen::Title => vec!["Novo caso".into(), "Novo caso — modo difícil: Relator Terrivelmente Evangélico".into(), "Dossiês: os casos reais".into()],
            Screen::DossieList => CASES.iter().map(|c| c.title.to_string()).chain(["Voltar".to_string()]).collect(),
            Screen::Dossie(_) => vec!["Voltar".into()],
            Screen::Brief => vec!["Começar a perseguição".into()],
            Screen::City => {
                let mut o: Vec<String> = self.witnesses.iter().map(|&w| format!("Interrogar: {}", witness_name(w))).collect();
                o.push("Viajar".into());
                if !self.mandado { o.push(format!("Pedir mandado ao STF ({}h)", H_WARRANT)); }
                o
            }
            Screen::Travel => self.dests.iter().map(|&d| format!("{} ({}h)", CITIES[d].name, flight(self.here, d))).chain(["Ficar aqui".to_string()]).collect(),
            Screen::End => vec!["Ver dossiê do caso".into(), "Novo caso".into(), "Menu".into()],
        }
    }

    pub fn act(&mut self, i: usize) {
        if i >= self.options().len() { return; }
        match self.screen {
            Screen::Title => match i {
                0 => self.new_case(false),
                1 => self.new_case(true),
                _ => self.screen = Screen::DossieList,
            },
            Screen::DossieList => self.screen = if i < CASES.len() { Screen::Dossie(i) } else { Screen::Title },
            Screen::Dossie(_) => self.screen = Screen::DossieList,
            Screen::Brief => self.screen = Screen::City,
            Screen::City => match i {
                0..=2 => self.ask(i),
                3 => self.open_travel(),
                _ => self.warrant(),
            },
            Screen::Travel => if i < self.dests.len() { self.travel(self.dests[i]) } else { self.screen = Screen::City },
            Screen::End => match i {
                0 => self.screen = Screen::Dossie(self.case),
                1 => self.new_case(self.hard),
                _ => self.screen = Screen::Title,
            },
        }
    }

    // botão à parte no JS: volta ao menu sem contar vitória nem derrota
    pub fn abandon(&mut self) {
        if matches!(self.screen, Screen::Brief | Screen::City | Screen::Travel) { self.screen = Screen::Title; }
    }

    fn header(&self) -> String {
        format!("{}👮 {}   📍 {}   ⏳ {}h   📎 Provas: {}   ⚖️ Mandado: {}\n\n", if self.hard { "⛪ " } else { "" }, self.posto().abbr,
            CITIES[self.here].name, self.hours, self.provas, if self.mandado { "sim" } else { "não" })
    }

    fn career(&self) -> String {
        let next = RANKS.get(rank(self.solved) + 1)
            .map_or(String::new(), |n| format!(" (falta {} para {})", n.need - self.solved, n.name));
        let s = if self.solved == 1 { "" } else { "s" };
        format!("\n\nPosto: {} · {} caso{s} resolvido{s}{}", self.posto().name, self.solved, next)
    }

    // Texto para o jogador copiar no fim do caso, com o link do jogo.
    fn share(&self) -> String {
        let city = CITIES[self.here].name;
        let line = if self.end.contains("🏆") { format!("🚔 Peguei o 01 em {} com mandado do STF e {}h de sobra", city, self.hours) }
            else if self.end.contains("ARQUIVADO") { format!("📁 O relator arquivou meu caso em {}", city) }
            else if self.end.contains("⏰") { format!("⏰ O prazo acabou e o 01 sumiu perto de {}", city) }
            else { format!("🛩️ Achei o 01 em {}, mas sem mandado ele embarcou num jatinho", city) };
        format!("{}{}. Cadê o 01? https://canducci.github.io/cade-o-01/", line, if self.hard { " ⛪" } else { "" })
    }

    // Primeira linha "@cena args" diz ao JS o que desenhar.
    fn scene(&self) -> String {
        match self.screen {
            Screen::Title => "@title".into(),
            Screen::DossieList | Screen::Dossie(_) => "@dossie".into(),
            Screen::Brief => format!("@brief {}", self.here),
            // @city cidade w0 w1 w2 quem_fala no_rastro horas prazo mandado
            Screen::City => format!("@city {} {} {} {} {} {} {} {} {}", self.here, self.witnesses[0], self.witnesses[1], self.witnesses[2],
                self.last_ask.map_or(0, |s| s + 1), self.on_trail() as u8, self.hours.max(0), self.deadline(), self.mandado as u8),
            Screen::Travel => format!("@travel {} {}", self.here, self.dests.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(" ")),
            // @win/@lose cidade casos_resolvidos arquivado (o JS salva o total)
            Screen::End => format!("@{} {} {} {}", if self.end.contains("🏆") { "win" } else { "lose" }, self.here, self.solved,
                self.end.contains("ARQUIVADO") as u8),
        }
    }

    pub fn render(&self) -> String {
        let mut s = self.scene() + "\n";
        s += &match self.screen {
            Screen::Title => "CADÊ O 01? Uma perseguição satírica pelo mundo.\n\
⚠️ PARÓDIA. Personagem inspirado em figura pública real. As acusações são descritas conforme reportagens, \
com o status de cada caso na seção Dossiês. Não há condenação nos casos citados e vale a presunção de inocência. \
Pistas, testemunhas e fugas no jogo são ficção.\n\n\
Você é agente da Divisão de Fugas Improváveis. Siga as pistas, junte provas, peça o mandado e alcance o 01 antes do prazo.".to_string()
                + &self.career(),
            Screen::DossieList => "DOSSIÊS\nO que dizem as investigações e reportagens (status em 28/10/2026).".into(),
            Screen::Dossie(i) => CASES[i].dossie.into(),
            Screen::Brief => format!("CASO: {}\n\n{}\n\nPonto de partida: {}. Posto: {}. Você tem {}h.\n\
Interrogar: {}h · Viajar: 1h a 15h, pela distância · Mandado: {}h, com {} provas, antes de alcançar o 01.{}",
                CASES[self.case].title, CASES[self.case].brief, CITIES[self.route[0]].name, self.posto().name, self.hours, H_ASK, H_WARRANT, NEED,
                if self.hard { format!("\n\n⛪ Modo difícil: 1 em {} viagens arquiva o caso, e metade das provas tende a ser anulada.", ARCHIVE_ODDS) } else { " Algumas provas podem ser anuladas.".into() }),
            Screen::City => format!("{}{}\n\n{}", self.header(), CITIES[self.here].desc, self.msg),
            Screen::Travel => format!("{}Para onde?", self.header()),
            Screen::End => format!("CASO: {}\n\n{}", CASES[self.case].title, self.end),
        };
        // linhas "~ " = links (veículo|URL) do dossiê; o JS vira botões
        // na cidade: "\" " = fala da testemunha (vai pro balão), "+ "/"- " = notícia boa/ruim (vira faixa)
        if let Screen::Dossie(i) = self.screen {
            for (label, url) in CASES[i].fontes {
                s += &format!("\n~ {}|{}", label, url);
            }
        }
        // linha "# " = texto de compartilhar; o JS tira do corpo e põe num botão
        if self.screen == Screen::End {
            s += "\n# ";
            s += &self.share();
        }
        for o in self.options() {
            s += "\n> ";
            s += &o;
        }
        s
    }
}

// --- ponte wasm: o JS chama start/act e lê o texto em out_ptr/out_len ---

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new(1, 0));
    static OUT: RefCell<String> = RefCell::new(String::new());
}

fn publish() {
    GAME.with(|g| OUT.with(|o| *o.borrow_mut() = g.borrow().render()));
}

#[no_mangle]
pub extern "C" fn start(seed: u32, solved: u32) {
    GAME.with(|g| *g.borrow_mut() = Game::new(seed, solved));
    publish();
}

#[no_mangle]
pub extern "C" fn act(i: u32) {
    GAME.with(|g| g.borrow_mut().act(i as usize));
    publish();
}

#[no_mangle]
pub extern "C" fn abandon() {
    GAME.with(|g| g.borrow_mut().abandon());
    publish();
}

#[no_mangle]
pub extern "C" fn out_ptr() -> *const u8 { OUT.with(|o| o.borrow().as_ptr()) }

#[no_mangle]
pub extern "C" fn out_len() -> usize { OUT.with(|o| o.borrow().len()) }

#[cfg(test)]
mod tests {
    use super::*;

    fn begin(seed: u32) -> Game { begin_mode(seed, 0) }

    // opt 0 = normal, 1 = difícil
    fn begin_mode(seed: u32, opt: usize) -> Game { begin_at(seed, opt, 0) }

    fn begin_at(seed: u32, opt: usize, solved: u32) -> Game {
        let mut g = Game::new(seed, solved);
        g.act(opt); // novo caso
        g.act(0); // começar
        g
    }

    // Joga o caminho certo: interroga todos, viaja pro destino correto, pede mandado no fim.
    fn play_perfect(seed: u32, opt: usize) -> String { play_perfect_at(seed, opt, 0).render() }

    fn play_perfect_at(seed: u32, opt: usize, solved: u32) -> Game {
        let mut g = begin_at(seed, opt, solved);
        while g.step < g.last() && g.screen != Screen::End {
            for s in 0..3 { g.act(s); }
            g.act(3);
            let next = g.route[g.step + 1];
            let i = g.dests.iter().position(|&d| d == next).expect("destino certo entre as opções");
            assert_eq!(g.dests.len(), 4);
            g.act(i);
        }
        if g.screen == Screen::End { return g; } // arquivado no caminho
        g.act(4); // mandado
        for s in 0..3 {
            g.act(s); // confronta com a primeira testemunha que não seja apoiadora
            if g.screen == Screen::End { break; }
        }
        assert_eq!(g.screen, Screen::End);
        g
    }

    // total de casos que a cena de fim manda o JS salvar
    fn end_total(g: &Game) -> u32 {
        g.render().lines().next().unwrap().split(' ').nth(2).unwrap().parse().unwrap()
    }

    #[test]
    fn rank_sets_deadline() {
        let game = |solved| { let mut g = Game::new(1, solved); g.act(0); g };
        let brief = |solved| game(solved).render();
        let (low, top) = (game(0).deadline(), game(10).deadline());
        assert_eq!(low - top, 40 - 16);
        assert!(brief(0).contains("Estagiário") && brief(0).contains(&format!("Você tem {}h", low)), "{}", brief(0));
        assert!(brief(9).contains("Delegado,") || brief(9).contains("Delegado."), "{}", brief(9));
        assert!(brief(10).contains("Delegado-chefe") && brief(10).contains(&format!("Você tem {}h", top)), "{}", brief(10));
        assert!(brief(u32::MAX).contains("Delegado-chefe"));
        let title = Game::new(1, 2).render();
        assert!(title.contains("Agente · 2 casos resolvidos (falta 1 para Agente especial)"), "{}", title);
    }

    #[test]
    fn wins_count_and_promote() {
        let won = |opt, solved| (1..200).map(|s| play_perfect_at(s, opt, solved))
            .find(|g| g.render().contains("CASO ENCERRADO")).unwrap();
        assert_eq!(end_total(&won(0, 0)), 1);
        assert!(won(0, 0).render().contains("⭐ Promovido a Agente!"));
        assert!(!won(0, 1).render().contains("Promovido"));
        assert_eq!(end_total(&won(1, 5)), 7);
        assert!(won(1, 5).render().contains("⭐ Promovido a Delegado!"));
        let lost = (1..200).map(|s| play_perfect_at(s, 0, 4)).find(|g| !g.render().contains("CASO ENCERRADO")).unwrap();
        assert_eq!(end_total(&lost), 4);
    }

    #[test]
    fn top_rank_still_winnable() {
        let wins = (1..100).filter(|&s| play_perfect_at(s, 0, 10).render().contains("CASO ENCERRADO")).count();
        assert!(wins > 40, "wins={}", wins);
    }

    #[test]
    fn perfect_play_can_win() {
        let wins = (1..50).filter(|&s| play_perfect(s, 0).contains("CASO ENCERRADO")).count();
        assert!(wins > 20, "wins={}", wins);
    }

    #[test]
    fn hard_mode_archives_and_wins_less() {
        let ends = |opt| (1..200).map(|s| play_perfect(s, opt)).collect::<Vec<_>>();
        let (normal, hard) = (ends(0), ends(1));
        let wins = |e: &[String]| e.iter().filter(|t| t.contains("CASO ENCERRADO")).count();
        assert!(hard.iter().any(|t| t.contains("ARQUIVADO")));
        assert!(!normal.iter().any(|t| t.contains("ARQUIVADO")));
        let share = |t: &String| t.lines().find(|l| l.starts_with("# ")).unwrap().to_string();
        assert!(hard.iter().all(|t| share(t).ends_with("⛪. Cadê o 01? https://canducci.github.io/cade-o-01/")));
        assert!(normal.iter().any(|t| share(t).starts_with("# 🚔 Peguei o 01 em ") && share(t).contains("h de sobra. ")));
        assert!(hard.iter().any(|t| share(t).starts_with("# 📁 O relator arquivou")));
        // a cena de fim marca o arquivamento para o JS desenhar o carimbo
        assert!(hard.iter().all(|t| t.contains("ARQUIVADO") == t.lines().next().unwrap().ends_with(" 1")));
        assert!(wins(&hard) > 0 && wins(&hard) < wins(&normal), "hard={} normal={}", wins(&hard), wins(&normal));
    }

    #[test]
    fn wrong_city_has_no_clues_and_time_runs_out() {
        let mut g = begin(7);
        g.act(3);
        let next = g.route[1];
        let i = g.dests.iter().position(|&d| d != next).unwrap();
        g.act(i);
        assert!(!g.on_trail());
        g.act(0);
        assert!(NOTHING.iter().any(|n| g.msg.contains(n)), "{}", g.msg);
        let w = g.witnesses[0];
        assert_eq!(w >= WITNESSES.len(), g.msg.contains(hello(w).trim()) && !hello(w).is_empty(), "{}", g.msg);
        // Volta é oferecida a partir da cidade errada
        g.act(3);
        assert!(g.dests.contains(&g.route[g.step]));
        g.act(4);
        while g.screen != Screen::End { g.act(0); }
        assert!(g.end.contains("prazo"));
    }

    #[test]
    fn witnesses_are_two_locals_and_one_generic() {
        let mut g = Game::new(3, 0);
        for city in 0..CITIES.len() {
            g.arrive(city);
            let locals: Vec<_> = g.witnesses.iter().filter(|&&w| w >= WITNESSES.len()).collect();
            assert_eq!(locals.len(), 2);
            assert!(locals.iter().all(|&&w| (w - WITNESSES.len()) / 3 == city));
            assert_ne!(locals[0], locals[1]);
        }
    }

    #[test]
    fn proof_is_never_assigned_to_a_supporter() {
        for seed in 1..100 {
            let mut g = Game::new(seed, 10);
            for city in 0..CITIES.len() {
                g.support_seen.clear();
                g.arrive(city);
                assert_ne!(g.supporter, Some(g.prova_slot));
                assert!((0..3).any(|slot| Some(slot) != g.supporter));
            }
        }
    }

    #[test]
    fn fixed_route_for_passport_case() {
        let g = (1..).map(begin).find(|g| g.case == 7).unwrap();
        assert_eq!(g.route, vec![BSB, BUE, GYE, MIA]);
        // voos 4h + 6h + 4h, 4 cidades x 3 interrogatórios x 2h, mandado 3h, folga do Estagiário 40h
        assert_eq!(g.deadline(), 14 + 24 + 3 + 40);
        assert_eq!(g.hours, g.deadline());
    }

    #[test]
    fn abandon_returns_to_menu_without_changing_career() {
        let mut g = begin_at(1, 0, 3);
        g.abandon();
        assert_eq!(g.screen, Screen::Title);
        assert_eq!(g.solved, 3);
        assert!(g.render().contains("3 casos resolvidos"));
        // na abertura e na viagem também; no fim de caso não faz nada
        let mut g = Game::new(1, 0);
        g.act(0);
        g.abandon();
        assert_eq!(g.screen, Screen::Title);
        let mut g = begin(1);
        g.act(3);
        g.abandon();
        assert_eq!(g.screen, Screen::Title);
        g.screen = Screen::End;
        g.abandon();
        assert_eq!(g.screen, Screen::End);
    }

    #[test]
    fn every_dossier_links_its_sources() {
        for c in CASES {
            assert!(!c.fontes.is_empty(), "{}", c.title);
            assert!(c.fontes.iter().all(|(l, u)| u.starts_with("https://") && !l.contains('|')), "{}", c.title);
            assert!(!c.dossie.contains("Fonte"), "{}", c.title);
        }
        let mut g = Game::new(1, 0);
        g.act(2);
        g.act(7);
        let links: Vec<_> = g.render().lines().filter(|l| l.starts_with("~ ")).map(String::from).collect();
        assert_eq!(links.len(), CASES[7].fontes.len());
        assert!(links[0].starts_with("~ Agência Brasil, 26/10/2026|https://agenciabrasil.ebc.com.br/"));
    }

    #[test]
    fn high_rank_gets_vague_clues_and_near_decoys() {
        for seed in 1..50 {
            for solved in [0, 6] {
                let mut g = begin_at(seed, 0, solved);
                let witness = (0..3).find(|&slot| g.supporter != Some(slot)).unwrap();
                g.act(witness);
                let next = &CITIES[g.route[1]];
                assert_eq!(solved == 6, next.vague.iter().any(|v| g.msg.contains(v)), "{}", g.msg);
                g.act(3);
                if solved == 6 {
                    let c = g.route[1];
                    let mut near: Vec<_> = (0..CITIES.len()).filter(|&x| x != g.here && x != c).collect();
                    near.sort_by_key(|&x| flight(c, x));
                    assert!(g.dests.iter().all(|d| *d == c || near[..5].contains(d)), "{:?}", g.dests);
                }
            }
        }
    }

    #[test]
    fn flight_hours_follow_distance() {
        assert_eq!(flight(SP, RIO), 1);
        assert_eq!(flight(BSB, MIA), 7);
        assert_eq!(flight(BUE, 8), 15); // Jerusalém
        assert_eq!(flight(RIO, SP), flight(SP, RIO));
        let mut g = begin(5);
        g.act(3);
        let d = g.dests[0];
        assert!(g.options()[0].ends_with(&format!(" ({}h)", flight(g.here, d))));
    }

    #[test]
    fn events_have_effects() {
        let mut seen = [false; 3];
        for seed in 1..200 {
            for mandado in [false, true] {
                let mut g = begin(seed);
                g.mandado = mandado;
                let (h, p) = (g.hours, g.provas);
                g.event();
                assert!(g.msg.starts_with("\n+ ") || g.msg.starts_with("\n- "), "{:?}", g.msg);
                if g.msg.contains("atrasado") { seen[0] = true; assert_eq!(g.hours, h - H_DELAY); }
                else if g.msg.contains("Habeas") { seen[1] = true; assert!(mandado && !g.mandado); }
                else { seen[2] = true; assert!(g.msg.contains("Vazamento")); assert_eq!(g.provas, p + 1); }
            }
        }
        assert_eq!(seen, [true; 3]);
    }

    #[test]
    fn end_speech_matches_result() {
        let ends = (1..200).flat_map(|s| [play_perfect(s, 0), play_perfect(s, 1)]).collect::<Vec<_>>();
        let pool = |t: &str| if t.contains("🏆") { END_WIN } else if t.contains("ARQUIVADO") { END_ARCHIVE }
            else if t.contains("⏰") { END_TIME } else { END_ESCAPE };
        for t in &ends {
            let speech = t.lines().find(|l| l.starts_with("\" ")).expect("fala no fim");
            let (who, line) = speech[2..].split_once(": \"").unwrap();
            assert!(pool(t).contains(&format!("{}: {}", who, line.trim_end_matches('"')).as_str()), "{}", speech);
        }
        let mut g = begin(5);
        g.provas = 9;
        assert!(g.render().lines().next().unwrap().ends_with(" 0"));
        while !g.mandado { g.warrant(); }
        assert!(g.render().lines().next().unwrap().ends_with(" 1"));
    }
}
