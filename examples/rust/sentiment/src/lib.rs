use serde::{ser::SerializeMap, Serialize, Serializer};

wit_bindgen_rust::export!("sentiment.wit");

impl Serialize for sentiment::PolarityScores {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("compound", &self.compound)?;
        map.serialize_entry("pos", &self.positive)?;
        map.serialize_entry("neg", &self.negative)?;
        map.serialize_entry("neu", &self.neutral)?;
        map.end()
    }
}

struct Sentiment;

#[debugger_macro::export_debug_handler]
impl sentiment::Sentiment for Sentiment {
    fn sentiment(input: String) -> sentiment::PolarityScores {
        lazy_static::lazy_static! {
            static ref ANALYZER: vader_sentiment::SentimentIntensityAnalyzer<'static> =
                vader_sentiment::SentimentIntensityAnalyzer::new();
        }

        let scores = ANALYZER.polarity_scores(input.as_str());
        sentiment::PolarityScores {
            compound: scores["compound"],
            positive: scores["pos"],
            negative: scores["neg"],
            neutral: scores["neu"],
        }
    }
}
