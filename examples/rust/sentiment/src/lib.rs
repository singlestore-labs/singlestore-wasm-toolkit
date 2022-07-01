wit_bindgen_rust::export!("sentiment.wit");
struct Sentiment;
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
