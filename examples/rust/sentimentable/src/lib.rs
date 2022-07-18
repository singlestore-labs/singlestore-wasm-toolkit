wit_bindgen_rust::export!("sentimentable.wit");
use crate::sentimentable::PolarityScores;
struct Sentimentable;
impl sentimentable::Sentimentable for Sentimentable {

    fn sentimentable(input: String) -> Vec<PolarityScores> {
        lazy_static::lazy_static! {
            static ref ANALYZER: vader_sentiment::SentimentIntensityAnalyzer<'static> =
                vader_sentiment::SentimentIntensityAnalyzer::new();
        }

        let scores = ANALYZER.polarity_scores(input.as_str());
        vec![PolarityScores {
            compound: scores["compound"],
            positive: scores["pos"],
            negative: scores["neg"],
            neutral: scores["neu"],
        }]
    }
}
