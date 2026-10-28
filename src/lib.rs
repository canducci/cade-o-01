// Cadê o 01? — paródia de perseguição geográfica. Lógica toda aqui; o JS só desenha texto e botões.
use std::cell::RefCell;

struct City {
    name: &'static str,
    desc: &'static str,
    people: [&'static str; 3], // testemunhas locais
    clues: [&'static str; 4],
}

const BSB: usize = 0;
const SP: usize = 1;
const RIO: usize = 2;
const BUE: usize = 3;
const GYE: usize = 4;
const MIA: usize = 5;

const CITIES: &[City] = &[
    City { name: "Brasília", desc: "Capital federal. Ar seco, Esplanada dos Ministérios e muito foro privilegiado.",
        people: ["Lobista do Congresso", "Segurança do Planalto", "Vendedor de pamonha"], clues: [
        "Disse que ia voltar pra cidade em forma de avião desenhada por Lúcio Costa.",
        "Perguntou se o Lago Paranoá tinha vista pra alguma mansão de R$ 6 milhões.",
        "Queria chegar a tempo de uma sessão no Congresso Nacional.",
        "Reclamou da umidade de 12% do Planalto Central.",
    ]},
    City { name: "São Paulo", desc: "Maior cidade do país. Trânsito, garoa e banqueiros na Faria Lima.",
        people: ["Banqueiro da Faria Lima", "Pasteleiro da feira", "Motoboy"], clues: [
        "Falou que tinha reunião com banqueiros na Faria Lima.",
        "Perguntou onde ficava o MASP, na Avenida Paulista.",
        "Levou guarda-chuva por causa da garoa.",
        "Queria comer pastel de feira e ver um jogo no Morumbi.",
    ]},
    City { name: "Rio de Janeiro", desc: "Cidade Maravilhosa. Cristo Redentor, praia e a velha ALERJ.",
        people: ["Vendedor de mate na praia", "Ex-servidor da ALERJ", "Surfista do Arpoador"], clues: [
        "Comentou saudade dos tempos de deputado na ALERJ.",
        "Queria ver o Cristo Redentor de perto.",
        "Falou de uma loja de chocolates num shopping da Barra da Tijuca.",
        "Ia assistir a um jogo no Maracanã.",
    ]},
    City { name: "Buenos Aires", desc: "Capital argentina. Tango, parrilla e Casa Rosada.",
        people: ["Dançarina de tango", "Parrillero de San Telmo", "Cambista do dólar blue"], clues: [
        "Trocou reais por pesos argentinos.",
        "Queria tirar foto no Obelisco da Avenida 9 de Julio.",
        "Perguntou o caminho para a Casa Rosada.",
        "Pediu doce de leite e uma aula de tango.",
    ]},
    City { name: "Guayaquil", desc: "Maior cidade do Equador, porto no Pacífico. Calor e iguanas no parque.",
        people: ["Vendedor de chapéu-panamá", "Guia do Malecón", "Atendente do aeroporto"], clues: [
        "Comemorou que lá usam dólar americano, mas se fala espanhol.",
        "Queria passear no Malecón 2000, à beira do rio Guayas.",
        "Perguntou se dava pra ver iguanas soltas num parque do centro.",
        "Disse que ia pra um país com nome de linha imaginária.",
    ]},
    City { name: "Miami", desc: "Flórida. Praia, Little Havana e brasileiros comprando eletrônicos.",
        people: ["Salva-vidas de South Beach", "Barista de Little Havana", "Brasileiro com seis malas"], clues: [
        "Comprou óculos escuros pra ir a South Beach.",
        "Quis tomar um cafecito em Little Havana.",
        "Disse que ia pra Flórida, mas não pra ver o Mickey.",
        "Perguntou quanto tempo de carro até Mar-a-Lago, em Palm Beach.",
    ]},
    City { name: "Orlando", desc: "Flórida central. Parques temáticos e filas intermináveis.",
        people: ["Pessoa fantasiada de mascote", "Vendedor de ingressos", "Brasileira na fila da montanha-russa"], clues: [
        "Queria tirar foto com um camundongo famoso.",
        "Perguntou sobre ingressos para parques temáticos.",
        "Ia fazer compras nos outlets da Flórida, longe da praia.",
        "Disse que ia pra capital mundial das montanhas-russas.",
    ]},
    City { name: "Washington", desc: "Capital dos EUA. Casa Branca, Capitólio e lobistas.",
        people: ["Lobista da K Street", "Guarda do Capitólio", "Estagiário do Depto. de Estado"], clues: [
        "Tinha reunião marcada no Departamento de Estado.",
        "Queria ver a Casa Branca pessoalmente.",
        "Perguntou onde fica o Capitólio.",
        "Disse que ia pra uma cidade com um obelisco gigante e muitos lobistas.",
    ]},
    City { name: "Jerusalém", desc: "Cidade sagrada para três religiões, em Israel.",
        people: ["Guia da Cidade Velha", "Vendedor de falafel", "Peregrino"], clues: [
        "Trocou dinheiro por shekels.",
        "Queria rezar no Muro das Lamentações.",
        "Disse que ia a um país com a Estrela de Davi na bandeira.",
        "Estava treinando o 'shalom'.",
    ]},
    City { name: "Roma", desc: "Capital da Itália. Coliseu, Vaticano e burocracia consular.",
        people: ["Atendente da cidadania italiana", "Gladiador de foto", "Guarda suíço"], clues: [
        "Mostrou orgulhoso o passaporte italiano.",
        "Queria ver o Coliseu.",
        "Perguntou se o Papa recebia visitas.",
        "Pagou em euro e pediu um espresso.",
    ]},
    City { name: "Lisboa", desc: "Capital de Portugal. Pastel de nata, elétrico 28 e muitos brasileiros.",
        people: ["Motorneiro do elétrico 28", "Fadista de Alfama", "Pasteleiro de Belém"], clues: [
        "Queria comer pastel de Belém.",
        "Perguntou como pegar o elétrico 28.",
        "Disse que ia pra onde falam português 'com sotaque'.",
        "Ia ouvir fado em Alfama.",
    ]},
    City { name: "Funchal", desc: "Ilha da Madeira, Portugal. Vinho, levadas e empresas offshore.",
        people: ["Contador de offshore", "Guia das levadas", "Produtor de vinho Madeira"], clues: [
        "Disse que ia pra uma ilha portuguesa no meio do Atlântico.",
        "Queria provar vinho Madeira.",
        "Perguntou pela terra natal do Cristiano Ronaldo.",
        "Comentou de uma holding registrada numa ilha portuguesa.",
    ]},
];

struct Case {
    title: &'static str,
    brief: &'static str,
    start: usize,
    fixed: &'static [usize],
    dossie: &'static str,
}

const CASES: &[Case] = &[
    Case { title: "Rachadinha na ALERJ", start: RIO, fixed: &[],
        brief: "Segundo o MP-RJ, pedaços dos salários de assessores da ALERJ tomaram rumo ignorado. O 01 também tomou rumo ignorado. Coincidência?",
        dossie: "RACHADINHA NA ALERJ\n\
Alegação: o MP-RJ denunciou Flávio em nov/2020 por peculato, lavagem de dinheiro e organização criminosa, apontando um esquema de recolhimento de salários de assessores da ALERJ operado por Fabrício Queiroz (R$ 6,1 mi, segundo o MP).\n\
Status: denúncia rejeitada em 2022, depois que STJ e STF anularam provas. O MP pediu a extinção; a decisão não impede nova investigação com provas válidas.\n\
Defesa: nega ter cometido crimes e diz que o caso está encerrado.\n\
Fontes: brasildefato.com.br (03/10/2026); infomoney.com.br" },
    Case { title: "Bombons Recheados", start: RIO, fixed: &[],
        brief: "Uma franquia de chocolates num shopping da Barra virou assunto do MP. O dono sumiu antes da Páscoa.",
        dossie: "FRANQUIA DE CHOCOLATES (KOPENHAGEN)\n\
Alegação: o MP-RJ apurou se a franquia comprada por Flávio e pelo sócio Alexandre Santini serviu para lavar até R$ 1,6 mi do suposto esquema da ALERJ.\n\
Status: linha de investigação do caso da rachadinha, cuja denúncia foi rejeitada em 2022. A loja foi entregue/fechada.\n\
Defesa: nega. \"Se eu quisesse lavar dinheiro, abriria uma franquia?\"\n\
Fontes: opovo.com.br (2020); folhape.com.br; poder360.com.br" },
    Case { title: "Imóveis no Dinheiro Vivo", start: RIO, fixed: &[],
        brief: "Corretores de Copacabana relatam pagamentos em espécie. O cliente pagou à vista e saiu à francesa.",
        dossie: "IMÓVEIS EM DINHEIRO VIVO\n\
Alegação: o MP-RJ apontou depósito de R$ 638,4 mil em espécie na conta de um corretor e suspeita de lucro artificial de R$ 800 mil com dois apartamentos em Copacabana. Levantamento do UOL listou 16 imóveis comprados com dinheiro vivo pela família (movimentação acima de R$ 3 mi).\n\
Status: parte do caso da rachadinha (denúncia rejeitada em 2022). Transação em espécie não é crime por si só.\n\
Defesa: nega irregularidades.\n\
Fontes: infomoney.com.br; brasildefato.com.br (03/10/2026)" },
    Case { title: "A Mansão do Lago", start: BSB, fixed: &[],
        brief: "Uma mansão de R$ 6 milhões em Brasília está com as luzes apagadas. O morador saiu pra comprar pão. Em outro continente.",
        dossie: "MANSÃO EM BRASÍLIA\n\
Fato: em 2021 Flávio comprou uma casa de cerca de R$ 6 mi em Brasília, com financiamento bancário. A compra foi questionada na imprensa.\n\
Status: nas fontes consultadas não há acusação criminal sobre a compra.\n\
Defesa: nega irregularidades na compra.\n\
Fonte: poder360.com.br" },
    Case { title: "Dark Horse", start: SP, fixed: &[],
        brief: "A PF segue o dinheiro de um banqueiro para uma cinebiografia. O produtor-executivo informal sumiu antes dos créditos.",
        dossie: "DARK HORSE / VORCARO\n\
Alegação: a PF investiga pagamentos do ex-banqueiro Daniel Vorcaro (Banco Master) ao filme 'Dark Horse', sobre Jair Bolsonaro. Segundo o The Intercept, Flávio pediu dinheiro a Vorcaro; a PF o aponta como interlocutor direto. Suspeitas: lavagem, corrupção e evasão de divisas.\n\
Status: inquérito no STF aberto em jul/2026 (min. André Mendonça), em andamento.\n\
Defesa: nega vantagem indevida; diz que foi \"patrocínio privado para um filme privado\".\n\
Fontes: agenciabrasil.ebc.com.br (27/10/2026); brasildefato.com.br (03/10/2026)" },
    Case { title: "Aposta Perdida", start: BSB, fixed: &[],
        brief: "Um inquérito sobre bets cita o nome do senador. Ele apostou que não ia ficar pra ver o resultado.",
        dossie: "BETS / WILLER TOMAZ\n\
Alegação: inquérito sobre suposta lavagem de dinheiro do advogado Willer Tomaz em negócios com bets. O nome do senador foi citado por uma secretária, sobre uma conta ligada à Alcazar Holdings (Ilha da Madeira).\n\
Status: inquérito no STF (relator min. Alexandre de Moraes), em andamento.\n\
Defesa: o senador nega irregularidades.\n\
Fonte: agenciabrasil.ebc.com.br (27/10/2026)" },
    Case { title: "Emendas Misteriosas", start: BSB, fixed: &[],
        brief: "Emendas parlamentares foram parar em institutos 'inexistentes de fato'. O padrinho delas também ficou inexistente de fato.",
        dossie: "EMENDAS PARLAMENTARES\n\
Alegação: reportagens apontam emendas indicadas por Flávio que pagaram um instituto que a Receita classificou como \"inexistente de fato\" (Mesquita-RJ), um instituto ligado aos irmãos Brazão e empresas investigadas por fraude em licitação.\n\
Status: apurações da PF, TCE-RJ e CGU. Não há acusação criminal contra ele por esses pontos.\n\
Defesa: diz que o gabinete indica emendas mas não escolhe as empresas contratadas.\n\
Fonte: brasildefato.com.br (03/10/2026)" },
    Case { title: "O Passaporte Que Não Voltou", start: BSB, fixed: &[BSB, BUE, GYE, MIA],
        brief: "O STF mandou entregar os passaportes em 24 horas. Passaram-se 24 horas. Os passaportes não. O senador também não.",
        dossie: "PASSAPORTES / SAÍDA DO PAÍS\n\
Fatos: em 26/10/2026 Flávio foi proibido de deixar o país e intimado a entregar os passaportes em 24h. O prazo venceu sem entrega e os passaportes foram cancelados. Segundo reportagens, ele saiu pela Argentina, fez escala em Guayaquil (Equador) e seguiu para Miami. Relatos divergem sobre datas e rota. Ele tem cidadania italiana desde 2023.\n\
Status: fatos em desenvolvimento (28/10/2026). Os dois inquéritos no STF continuam.\n\
Fontes: agenciabrasil.ebc.com.br (27/10/2026); nexojornal.com.br (27/10/2026)" },
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

// carreira: casos resolvidos para chegar ao posto e prazo de cada caso nele
// 64h é o piso: 5 cidades jogadas sem erro, interrogando todos, gastam 61h.
struct Posto { need: u32, name: &'static str, abbr: &'static str, hours: i32 }
const RANKS: [Posto; 5] = [
    Posto { need: 0, name: "Estagiário", abbr: "Estag.", hours: 80 },
    Posto { need: 1, name: "Agente", abbr: "Ag.", hours: 76 },
    Posto { need: 3, name: "Agente especial", abbr: "Ag. esp.", hours: 72 },
    Posto { need: 6, name: "Delegado", abbr: "Del.", hours: 68 },
    Posto { need: 10, name: "Delegado-chefe", abbr: "Del.-chefe", hours: 64 },
];
fn rank(solved: u32) -> usize { RANKS.iter().rposition(|r| solved >= r.need).unwrap() }
const H_ASK: i32 = 2;
const H_TRAVEL: i32 = 8;
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
    dests: Vec<usize>,
    msg: String,
    end: String,
}

impl Game {
    pub fn new(seed: u32, solved: u32) -> Game {
        Game {
            rng: seed as u64 | 1, screen: Screen::Title, case: 0, route: vec![], step: 0, here: 0,
            hours: RANKS[rank(solved)].hours, provas: 0, prova_slot: 0, prova_taken: false, last_ask: None, mandado: false, hard: false, solved,
            witnesses: [0, 1, 2], dests: vec![], msg: String::new(), end: String::new(),
        }
    }

    fn rand(&mut self, n: usize) -> usize {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng % n as u64) as usize
    }

    fn posto(&self) -> &'static Posto { &RANKS[rank(self.solved)] }
    fn deadline(&self) -> i32 { self.posto().hours }

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
        self.msg.clear();
        self.arrive(self.route[0]);
        self.screen = Screen::Brief;
    }

    fn arrive(&mut self, city: usize) {
        self.here = city;
        self.prova_slot = self.rand(3);
        self.prova_taken = false;
        self.last_ask = None;
        // duas testemunhas locais + uma genérica, em posições sorteadas
        let a = self.rand(3);
        let b = (a + 1 + self.rand(2)) % 3;
        let local = |k| WITNESSES.len() + city * 3 + k;
        let mut w = [local(a), local(b), self.rand(WITNESSES.len())];
        w.rotate_left(self.rand(3));
        self.witnesses = w;
    }

    fn finish(&mut self, text: String) {
        self.end = text;
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
        if !self.on_trail() {
            self.msg = format!("{}: \"{}{}\"", who, hi, NOTHING[self.rand(NOTHING.len())]);
        } else if self.step == self.last() {
            let city = CITIES[self.here].name;
            if self.mandado {
                let before = rank(self.solved);
                self.solved = self.solved.saturating_add(if self.hard { 2 } else { 1 });
                let now = rank(self.solved);
                let promo = if now > before { format!("\n\n⭐ Promovido a {}!", RANKS[now].name) } else { String::new() };
                self.finish(format!("🚔 {} aponta uma esquina de {}. Lá está ele, de boné e óculos escuros.\nVocê mostra o mandado do STF.\n\"Isso é perseguição!\", declara o 01, antes de pedir pra falar com o advogado.\n\n🏆 CASO ENCERRADO (por enquanto — ainda cabe recurso).{}", who, city, promo));
            } else {
                self.finish(format!("🛩️ {} aponta uma esquina de {}. Lá está ele!\nMas você não tem mandado. Ele acena, sorri e embarca num jatinho.\n\n❌ Sem mandado, sem prisão. Junte provas e peça ao STF antes do confronto.", who, city));
            }
        } else {
            let next = self.route[self.step + 1];
            let clue = CITIES[next].clues[self.rand(4)];
            self.msg = format!("{}: \"{}{}\"", who, hi, clue);
            if slot == self.prova_slot && !self.prova_taken {
                self.prova_taken = true;
                self.provas += 1;
                let p = PROVAS[self.rand(PROVAS.len())];
                self.msg += &format!("\n📎 De quebra, entregou {}. (+1 prova)", p);
            }
        }
        self.check_time();
    }

    fn open_travel(&mut self) {
        let correct = if !self.on_trail() { Some(self.route[self.step]) }
            else if self.step < self.last() { Some(self.route[self.step + 1]) }
            else { None };
        let mut d: Vec<usize> = correct.into_iter().collect();
        while d.len() < 4 {
            let x = self.rand(CITIES.len());
            if x != self.here && !d.contains(&x) { d.push(x); }
        }
        for i in (1..d.len()).rev() {
            let j = self.rand(i + 1);
            d.swap(i, j);
        }
        self.dests = d;
        self.screen = Screen::Travel;
    }

    fn travel(&mut self, dest: usize) {
        self.hours -= H_TRAVEL;
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
        self.screen = Screen::City;
        self.check_time();
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
            self.msg = if annulled > 0 { format!("🗑️ {} prova(s) anulada(s) por questões processuais. ", annulled) } else { String::new() };
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
            Screen::Travel => self.dests.iter().map(|&d| CITIES[d].name.to_string()).chain(["Ficar aqui".to_string()]).collect(),
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
        let line = if self.end.contains("🏆") { format!("🚔 Peguei o 01 em {} com {}h de sobra", city, self.hours) }
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
            // @city cidade w0 w1 w2 quem_fala no_rastro horas prazo
            Screen::City => format!("@city {} {} {} {} {} {} {} {}", self.here, self.witnesses[0], self.witnesses[1], self.witnesses[2],
                self.last_ask.map_or(0, |s| s + 1), self.on_trail() as u8, self.hours.max(0), self.deadline()),
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
Interrogar: {}h · Viajar: {}h · Mandado: {}h, com {} provas, antes de alcançar o 01.{}",
                CASES[self.case].title, CASES[self.case].brief, CITIES[self.route[0]].name, self.posto().name, self.hours, H_ASK, H_TRAVEL, H_WARRANT, NEED,
                if self.hard { format!("\n\n⛪ Modo difícil: 1 em {} viagens arquiva o caso, e metade das provas tende a ser anulada.", ARCHIVE_ODDS) } else { " Algumas provas podem ser anuladas.".into() }),
            Screen::City => format!("{}{}\n\n{}", self.header(), CITIES[self.here].desc, self.msg),
            Screen::Travel => format!("{}Para onde?", self.header()),
            Screen::End => format!("CASO: {}\n\n{}", CASES[self.case].title, self.end),
        };
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
        g.act(0); // confronto
        assert_eq!(g.screen, Screen::End);
        g
    }

    // total de casos que a cena de fim manda o JS salvar
    fn end_total(g: &Game) -> u32 {
        g.render().lines().next().unwrap().split(' ').nth(2).unwrap().parse().unwrap()
    }

    #[test]
    fn rank_sets_deadline() {
        let brief = |solved| { let mut g = Game::new(1, solved); g.act(0); g.render() };
        assert!(brief(0).contains("Estagiário") && brief(0).contains("Você tem 80h"), "{}", brief(0));
        assert!(brief(9).contains("Delegado,") || brief(9).contains("Delegado."), "{}", brief(9));
        assert!(brief(10).contains("Delegado-chefe") && brief(10).contains("Você tem 64h"), "{}", brief(10));
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
    fn fixed_route_for_passport_case() {
        let g = (1..).map(begin).find(|g| g.case == 7).unwrap();
        assert_eq!(g.route, vec![BSB, BUE, GYE, MIA]);
    }
}
