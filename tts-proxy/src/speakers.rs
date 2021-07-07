use std::collections::{HashMap, HashSet};

pub struct TtsSpeakers {
    pub speakers: HashMap<&'static str, &'static str>,
    pub speaker_ids: HashSet<&'static str>,
}

impl TtsSpeakers {
    /// Returns the speaker id for a given speaker name. If the name is a speaker id, returns a static reference to the string with the same id.
    pub fn get_speaker_id(&self, speaker_or_id: &str) -> Option<&'static str> {
        self.speakers
            .get(speaker_or_id)
            .or_else(|| self.speaker_ids.get(speaker_or_id))
            .copied()
    }
}

lazy_static::lazy_static! {
    pub static ref TTS: TtsSpeakers = {
        let mut speakers = HashMap::new();
        speakers.insert("altman", "sam-altman");
        speakers.insert("arnold", "arnold-schwarzenegger");
        speakers.insert("attenborough", "david-attenborough");
        speakers.insert("ayoade", "richard-ayoade");
        speakers.insert("barker", "bob-barker");
        speakers.insert("bart", "bart-simpson");
        speakers.insert("bill", "bill-clinton");
        speakers.insert("boss", "the-boss");
        speakers.insert("brimley", "wilford-brimley");
        speakers.insert("broomstick", "boomstick");
        speakers.insert("bush", "george-w-bush");
        speakers.insert("carter", "jimmy-carter");
        speakers.insert("cooper", "anderson-cooper");
        speakers.insert("cramer", "jim-cramer");
        speakers.insert("cranston", "bryan-cranston");
        speakers.insert("cross", "david-cross");
        speakers.insert("darth", "darth-vader");
        speakers.insert("deen", "paula-deen");
        speakers.insert("degrasse", "neil-degrasse-tyson");
        speakers.insert("dench", "judi-dench");
        speakers.insert("devito", "danny-devito");
        speakers.insert("ferguson", "craig-ferguson");
        speakers.insert("gates", "bill-gates");
        speakers.insert("gottfried", "gilbert-gottfried");
        speakers.insert("graham", "paul-graham");
        speakers.insert("hillary", "hillary-clinton");
        speakers.insert("homer", "homer-simpson");
        speakers.insert("jones", "james-earl-jones");
        speakers.insert("keeper", "crypt-keeper");
        speakers.insert("king", "larry-king");
        speakers.insert("krabs", "mr-krabs");
        speakers.insert("lee", "christopher-lee");
        speakers.insert("lisa", "lisa-simpson");
        speakers.insert("luckey", "palmer-luckey");
        speakers.insert("mcconnell", "mitch-mcconnell");
        speakers.insert("nimoy", "leonard-nimoy");
        speakers.insert("nixon", "richard-nixon");
        speakers.insert("nye", "bill-nye");
        speakers.insert("obama", "barack-obama");
        speakers.insert("oliver", "john-oliver");
        speakers.insert("palin", "sarah-palin");
        speakers.insert("penguinz0", "moistcr1tikal");
        speakers.insert("phil", "dr-phil-mcgraw");
        speakers.insert("reagan", "ronald-reagan");
        speakers.insert("rickman", "alan-rickman");
        speakers.insert("rogers", "fred-rogers");
        speakers.insert("rosen", "michael-rosen");
        speakers.insert("saruman", "saruman");
        speakers.insert("scout", "scout");
        speakers.insert("shapiro", "ben-shapiro");
        speakers.insert("shohreh", "shohreh-aghdashloo");
        speakers.insert("simmons", "j-k-simmons");
        speakers.insert("snake", "solid-snake");
        speakers.insert("snape", "severus-snape");
        speakers.insert("sonic", "sonic");
        speakers.insert("tails", "tails");
        speakers.insert("knuckles", "knuckles");
        speakers.insert("spongebob", "spongebob-squarepants");
        speakers.insert("squidward", "squidward");
        speakers.insert("stein", "ben-stein");
        speakers.insert("takei", "george-takei");
        speakers.insert("thiel", "peter-thiel");
        speakers.insert("trevor", "trevor-philips");
        speakers.insert("trump", "donald-trump");
        speakers.insert("tucker", "tucker-carlson");
        speakers.insert("tupac", "tupac-shakur");
        speakers.insert("vegeta", "vegeta");
        speakers.insert("goku", "goku");
        speakers.insert("white", "betty-white");
        speakers.insert("wiseau", "tommy-wiseau");
        speakers.insert("wizard", "wizard");
        speakers.insert("yugi", "yami-yugi");
        speakers.insert("zuckerberg", "mark-zuckerberg");


        let speaker_ids = speakers.values().copied().collect();
        TtsSpeakers { speakers, speaker_ids }
    };
}
