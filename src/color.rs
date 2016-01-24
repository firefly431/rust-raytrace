//! Struct representing color
//!
//! This module contains the `Color` struct representing the light
//! that is transferred by light sources. It is transformed into
//! RGB at the end of the rendering process.

use std::ops::{Add, Sub, Mul, Div};

/// A color which can be transformed into RGB components. Currently
/// colors are stored as three `f64` RGB components.
///
/// All components are normally in the range [0, 1], but colors can
/// go beyond that (e.g. when adding colors).
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Color {
    /// The red component of the color.
    pub r: f64,
    /// The green component of the color.
    pub g: f64,
    /// The blue component of the color.
    pub b: f64,
}

/// The color black.
pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {r: self.r + other.r, g: self.g + other.g, b: self.b + other.b}
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color {r: self.r - other.r, g: self.g - other.g, b: self.b - other.b}
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {r: self.r * other.r, g: self.g * other.g, b: self.b * other.b}
    }
}

impl Div for Color {
    type Output = Color;
    fn div(self, other: Color) -> Color {
        Color {r: self.r / other.r, g: self.g / other.g, b: self.b / other.b}
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color {r: self.r * other, g: self.g * other, b: self.b * other}
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, other: f64) -> Color {
        Color {r: self.r / other, g: self.g / other, b: self.b / other}
    }
}

fn clamp_color_val(val: f64) -> u8 {
    let x = val * 255.0;
    if x < 0.0 {0} else if x >= 255.0 {255} else {x.trunc() as u8}
}

// SRGB_VALUES[i] = the linear RGB value for that byte
const SRGB_VALUES: [f64; 256] = [
    0.000000000000000000000,
    0.000303526983548837515,
    0.000607053967097675030,
    0.000910580950646512491,
    0.001214107934195350061,
    0.001517634917744187413,
    0.001821161901293024982,
    0.002124688884841862552,
    0.002428215868390700121,
    0.002731742851939537257,
    0.003035269835488374826,
    0.003346535763899160815,
    0.003676507324047435885,
    0.004024717018496306621,
    0.004391442037410293354,
    0.004776953480693729194,
    0.005181516702338385960,
    0.005605391624202722863,
    0.006048833022857053911,
    0.006512090792594475187,
    0.006995410187265386874,
    0.007499032043226175337,
    0.008023192985384994258,
    0.008568125618069306892,
    0.009134058702220787182,
    0.009721217320237849138,
    0.010329823029626936451,
    0.010960094006488245791,
    0.011612245179743884912,
    0.012286488356915871781,
    0.012983032342173012399,
    0.013702083047289686374,
    0.014443843596092544729,
    0.015208514422912709427,
    0.015996293365509631212,
    0.016807375752887383780,
    0.017641954488384077593,
    0.018500220128379697010,
    0.019382360956935722890,
    0.020288563056652400562,
    0.021219010376003554635,
    0.022173884793387381442,
    0.023153366178110409979,
    0.024157632448504755968,
    0.025186859627361630337,
    0.026241221894849897645,
    0.027320891639074893625,
    0.028426039504420793497,
    0.029556834437808800209,
    0.030713443732993634533,
    0.031896033073011531567,
    0.033104766570885055255,
    0.034339806808682170336,
    0.035601314875020342865,
    0.036889450401100039312,
    0.038204371595346502122,
    0.039546235276732837061,
    0.040915196906853190661,
    0.042311410620809675187,
    0.043735029256973464995,
    0.045186204385675540762,
    0.046665086336880094720,
    0.048171824226889418952,
    0.049706565984127232261,
    0.051269458374043237747,
    0.052860647023180246107,
    0.054480276442442368556,
    0.056128490049600091027,
    0.057805430191067229406,
    0.059511238162981199007,
    0.061246054231617608199,
    0.063010017653167674223,
    0.064803266692905772683,
    0.066625938643772891767,
    0.068478169844400166300,
    0.070360095696595875703,
    0.072271850682317478887,
    0.074213568380149627646,
    0.076185381481307851148,
    0.078187421805186327339,
    0.080219820314468323619,
    0.082282707129814794400,
    0.084376211544148815857,
    0.086500462036549763400,
    0.088655586285772941535,
    0.090841711183407683472,
    0.093058962846687451331,
    0.095307466630964704501,
    0.097587347141862457178,
    0.099898728247113890988,
    0.102241733088101319216,
    0.104616484091104189336,
    0.107023102978267614649,
    0.109461710778299331492,
    0.111932427836905601382,
    0.114435373826973732503,
    0.116970667758510837864,
    0.119538427988345616337,
    0.122138772229601871855,
    0.124771817560950487591,
    0.127437680435647432420,
    0.130136476690364294440,
    0.132868321553817975156,
    0.135633329655205664421,
    0.138431615032451826863,
    0.141263291140271640689,
    0.144128470858057772253,
    0.147027266497594982786,
    0.149959789810608562499,
    0.152926151996150172518,
    0.155926463707827395178,
    0.158960835060880406600,
    0.162029375639110989615,
    0.165132194501667606268,
    0.168269400189690748748,
    0.171441100732822593322,
    0.174647403655585037319,
    0.177888415983629116779,
    0.181164244249860217906,
    0.184474994500440997447,
    0.187820772300677868438,
    0.191201682740791384374,
    0.194617830441575795364,
    0.198069319559948858744,
    0.201556253794397066681,
    0.205078736390316929139,
    0.208636870145255753917,
    0.212230757414055226651,
    0.215860500113899261843,
    0.219526199729269205774,
    0.223227957316808500732,
    0.226965873510098364862,
    0.230740048524349150583,
    0.234550582161005216619,
    0.238397573812271001970,
    0.242281122465554860090,
    0.246201326707835482788,
    0.250158284729953439562,
    0.254152094330826749946,
    0.258182852921595817897,
    0.262250657529696229453,
    0.266355604802862466762,
    0.270497791013065813637,
    0.274677312060384648529,
    0.278894263476810400082,
    0.283148740429992107348,
    0.287440837726917475248,
    0.291770649817535865367,
    0.296138270798321112665,
    0.300543794415776499562,
    0.304987314069886272794,
    0.309468922817508540479,
    0.313988713375717543030,
    0.318546778125091856193,
    0.323143209112950746675,
    0.327778098056542177563,
    0.332451536346179354897,
    0.337163615048330367330,
    0.341914424908660918856,
    0.346704056355029599512,
    0.351532599500439357776,
    0.356400144145943509333,
    0.361306779783509501858,
    0.366252595598839492119,
    0.371237680474149123189,
    0.376262122990906500153,
    0.381326011432530143086,
    0.386429433787049025906,
    0.391572477749723257823,
    0.396755230725626850674,
    0.401977779832195791787,
    0.407240211901736703926,
    0.412542613483903752858,
    0.417885070848137474275,
    0.423267669986071681798,
    0.428690496613906624201,
    0.434153636174748946974,
    0.439657173840918791274,
    0.445201194516227860554,
    0.450785782838223458846,
    0.456411023180404662458,
    0.462076999654407072349,
    0.467783796112158978264,
    0.473531496148009545255,
    0.479320183100826802125,
    0.485149940056070372307,
    0.491020849847835616497,
    0.496932995060870408288,
    0.502886458032568706145,
    0.508881320854933760778,
    0.514917665376521394016,
    0.520995573204354300856,
    0.527115125705813092338,
    0.533276404010505244990,
    0.539479489012107182866,
    0.545724461370186597620,
    0.552011401512000121627,
    0.558340389634267908470,
    0.564711505704929228600,
    0.571124829464873084994,
    0.577580440429650621148,
    0.584078417891164103359,
    0.590618840919336918205,
    0.597201788363763363954,
    0.603827338855337791834,
    0.610495570807864762486,
    0.617206562419651105778,
    0.623960391675076109230,
    0.630757136346146829453,
    0.637596873994032642408,
    0.644479681970582141126,
    0.651405637419824157242,
    0.658374817279448465435,
    0.665387298282272054983,
    0.672443156957687526720,
    0.679542469633093837444,
    0.686685312435313499613,
    0.693871761291989908038,
    0.701101891932973120269,
    0.708375779891686763179,
    0.715693500506480728696,
    0.723055128921969325617,
    0.730460740090353666254,
    0.737910408772730841420,
    0.745404209540387441280,
    0.752942216776077866136,
    0.760524504675292423173,
    0.768151147247506993487,
    0.775822218317423595302,
    0.783537791526193516667,
    0.791297940332630234117,
    0.799102738014409008649,
    0.806952257669251604710,
    0.814846572216101239228,
    0.822785754396283541823,
    0.830769876774654636442,
    0.838799011740740008136,
    0.846873231509858048760,
    0.854992608124233832712,
    0.863157213454102345551,
    0.871367119198797168700,
    0.879622396887831725643,
    0.887923117881966317277,
    0.896269353374266386503,
    0.904661174391149569551,
    0.913098651793419202605,
    0.921581856277294608759,
    0.930110858375423732447,
    0.938685728457888002296,
    0.947306536733199866518,
    0.955973353249286117439,
    0.964686247894465109809,
    0.973445290398412543809,
    0.982250550333117145030,
    0.991102097113829794139,
    1.000000000000000000000,
];

// average of two consecutive sRGB values
const SRGB_AVERAGE: [f64; 255] = [
    0.000151763491774418758,
    0.000455290475323256246,
    0.000758817458872093707,
    0.001062344442420931276,
    0.001365871425969768845,
    0.001669398409518606198,
    0.001972925393067443550,
    0.002276452376616281553,
    0.002579979360165118689,
    0.002883506343713955825,
    0.003190902799693767604,
    0.003511521543973298350,
    0.003850612171271871470,
    0.004208079527953299988,
    0.004584197759052011274,
    0.004979235091516057143,
    0.005393454163270553978,
    0.005827112323529888387,
    0.006280461907725764549,
    0.006753750489929931031,
    0.007247221115245781539,
    0.007761112514305584797,
    0.008295659301727150575,
    0.008851092160145047905,
    0.009427638011229318160,
    0.010025520174932392795,
    0.010644958518057591121,
    0.011286169593116065352,
    0.011949366768329878347,
    0.012634760349544442090,
    0.013342557694731350254,
    0.014072963321691115551,
    0.014826179009502626210,
    0.015602403894211171187,
    0.016401834559198507496,
    0.017224665120635730686,
    0.018071087308381889036,
    0.018941290542657709950,
    0.019835462006794063461,
    0.020753786716327979334,
    0.021696447584695466304,
    0.022663625485748895710,
    0.023655499313307584708,
    0.024672246037933193152,
    0.025714040761105762256,
    0.026781056766962393900,
    0.027873465571747843561,
    0.028991436971114795118,
    0.030135139085401219106,
    0.031304738403002581315,
    0.032500399821948293411,
    0.033722286689783609326,
    0.034970560841851253131,
    0.036245382638060194558,
    0.037546910998223270717,
    0.038875303436039669591,
    0.040230716091793017331,
    0.041613303763831432924,
    0.043023219938891570091,
    0.044460616821324502879,
    0.045925645361277814271,
    0.047418455281884756836,
    0.048939195105508329076,
    0.050488012179085231534,
    0.052065052698611738458,
    0.053670461732811307332,
    0.055304383246021229792,
    0.056966960120333656747,
    0.058658334177024214207,
    0.060378646197299407072,
    0.062128035942392641211,
    0.063906642173036723453,
    0.065714602668339339164,
    0.067552054244086529033,
    0.069419132770498021001,
    0.071315973189456677295,
    0.073242709531233546327,
    0.075199474930728732458,
    0.077186401643247082305,
    0.079203621059827325479,
    0.081251263722141559009,
    0.083329459336981798190,
    0.085438336790349289629,
    0.087578024161161352468,
    0.089748648734590319442,
    0.091950337015047567402,
    0.094183214738826070977,
    0.096447406886413580840,
    0.098743037694488167144,
    0.101070230667607605102,
    0.103429108589602747337,
    0.105819793534685901992,
    0.108242406878283473071,
    0.110697069307602466437,
    0.113183900831939660003,
    0.115703020792742278244,
    0.118254547873428234039,
    0.120838600108973737157,
    0.123455294895276179723,
    0.126104748998298960005,
    0.128787078563005863430,
    0.131502399122091134798,
    0.134250825604511819789,
    0.137032472343828759520,
    0.139847453086361733776,
    0.142695880999164692593,
    0.145577868677826377519,
    0.148493528154101772643,
    0.151442970903379381387,
    0.154426307851988797726,
    0.157443649384353900889,
    0.160495105349995698107,
    0.163580785070389311819,
    0.166700797345679163630,
    0.169855250461256657157,
    0.173044252194203829198,
    0.176267909819607077049,
    0.179526330116744681220,
    0.182819619375150621554,
    0.186147883400559432943,
    0.189511227520734626406,
    0.192909756591183589869,
    0.196343575000762327054,
    0.199812786677172948835,
    0.203317495092356997910,
    0.206857803267786355406,
    0.210433813779655476406,
    0.214045628763977258124,
    0.217693349921584233808,
    0.221377078523038839375,
    0.225096915413453446675,
    0.228852961017223743845,
    0.232645315342677183601,
    0.236474077986638109294,
    0.240339348138912944908,
    0.244241224586695171439,
    0.248179805718894475053,
    0.252155189530390066999,
    0.256167473626211283921,
    0.260216755225646023675,
    0.264303131166279348108,
    0.268426697907964140200,
    0.272587551536725203327,
    0.276785787768597524305,
    0.281021501953401253715,
    0.285294789078454791298,
    0.289605743772226698063,
    0.293954460307928489016,
    0.298341032607048806113,
    0.302765554242831358422,
    0.307228118443697406637,
    0.311728818096613013999,
    0.316267745750404727367,
    0.320844993619021301434,
    0.325460653584746462119,
    0.330114817201360766230,
    0.334807575697254833358,
    0.339539019978495670848,
    0.344309240631845259184,
    0.349118327927734450888,
    0.353966371823191461310,
    0.358853461964726505595,
    0.363779687691174524744,
    0.368745138036494335410,
    0.373749901732527811671,
    0.378794067211718321619,
    0.383877722609789584496,
    0.389000955768386114109,
    0.394163854237675082004,
    0.399366505278911321231,
    0.404608995866966247856,
    0.409891412692820256147,
    0.415213842166020641322,
    0.420576370417104605792,
    0.425979083299989125244,
    0.431422066394327785588,
    0.436905405007833869124,
    0.442429184178573353670,
    0.447993488677225659700,
    0.453598403009314088408,
    0.459244011417405895159,
    0.464930397883283053062,
    0.470657646130084261760,
    0.476425839624418201446,
    0.482235061578448587216,
    0.488085394951953022158,
    0.493976922454352984637,
    0.499909726546719557216,
    0.505883889443751177950,
    0.511899493115727577397,
    0.517956619290437902947,
    0.524055349455083696597,
    0.530195764858159224175,
    0.536377946511306213928,
    0.542601975191146834732,
    0.548867931441093359624,
    0.555175895573134070560,
    0.561525947669598624046,
    0.567918167584901212308,
    0.574352634947261853071,
    0.580829429160407362254,
    0.587348629405250566293,
    0.593910314641550085568,
    0.600514563609550577894,
    0.607161454831601332671,
    0.613851066613757989643,
    0.620583477047363607504,
    0.627358764010611524853,
    0.634177005170089680419,
    0.641038277982307391767,
    0.647942659695203149184,
    0.654890227349636311338,
    0.661881057780860260209,
    0.668915227619979790852,
    0.675992813295390737593,
    0.683113891034203613017,
    0.690278536863651703825,
    0.697486826612481514154,
    0.704738835912329886213,
    0.712034640199083801448,
    0.719374314714225082668,
    0.726757934506161440424,
    0.734185574431542198326,
    0.741657309156559141350,
    0.749173213158232709219,
    0.756733360725685200165,
    0.764337825961399763841,
    0.771986682782465294395,
    0.779680004921808555984,
    0.787417865929411875392,
    0.795200339173519621383,
    0.803027497841830362191,
    0.810899414942676477480,
    0.818816163306192335014,
    0.826777815585469033621,
    0.834784444257697266778,
    0.842836121625299083959,
    0.850932919817045885225,
    0.859074910789168089131,
    0.867262166326449701614,
    0.875494758043314447171,
    0.883772757384899021460,
    0.892096235628116351890,
    0.900465263882707978027,
    0.908879913092284441589,
    0.917340254035356905682,
    0.925846357326359115092,
    0.934398293416655922883,
    0.942996132595543934407,
    0.951639944991242936467,
    0.960329800571875669135,
    0.969065769146438826809,
    0.977847920365764844419,
    0.986676323723473469585,
    0.995551048556914897070,
];

fn to_srgb(val: f64) -> u8 {
    for i in 0..255 {
        if val < SRGB_AVERAGE[i] {
            return i as u8;
        }
    }
    return 255
}

impl Color {
    /// Creates a color from RGB components. The resulting color
    /// has components are that not clamped to [0, 1).
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Color {
        Color {r: r, g: g, b: b}
    }

    /// Creates a color from RGB components, where the components
    /// are sRGB bytes.
    pub fn from_srgb(r: u8, g: u8, b: u8) -> Color {
        Color { r: SRGB_VALUES[r as usize], g: SRGB_VALUES[g as usize], b: SRGB_VALUES[b as usize] }
    }

    /// Get the RGB components as 3 bytes, useful for writing an
    /// image.
    pub fn rgb(&self) -> [u8; 3] {
        [clamp_color_val(self.r), clamp_color_val(self.g), clamp_color_val(self.b)]
    }

    /// Get the components of the image in BGR as 3 bytes, useful
    /// for writing an image.
    pub fn bgr(&self) -> [u8; 3] {
        [clamp_color_val(self.b), clamp_color_val(self.g), clamp_color_val(self.r)]
    }

    /// Write this color to the ith position in the row buffer.
    pub fn write_bgr(&self, buf: &mut Vec<u8>, i: usize) {
        buf[i * 3]     = to_srgb(self.b);
        buf[i * 3 + 1] = to_srgb(self.g);
        buf[i * 3 + 2] = to_srgb(self.r);
    }

    /// Some indication of significance; if 0, unsignificant; if
    /// greater than 0, significant. Used to disable shading when
    /// unnecessary.
    pub fn significance(&self) -> f64 {
        self.r + self.g + self.b
    }
}
