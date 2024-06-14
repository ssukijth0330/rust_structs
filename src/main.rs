use core::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct MediaPlaylist {
    ended: bool,
    segments: Vec<MediaSegment>,
    target_duration: Duration,
    version: u64,
    discontinuity: Vec<DiscontinuitySegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaSegment {
    duration: Duration,
    url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscontinuitySegment {
    discontinuity_duration: Duration,
    discontinuity_segments: Vec<MediaSegment>,
}

fn main() {
    let mut media_playlist = MediaPlaylist {
        ended: false,
        segments: vec![],
        target_duration: Duration::new(0, 0),
        version: 0,
        discontinuity: vec![],
    };

    let media_segment1 = MediaSegment {
        duration: Duration::new(10, 1234567),
        url: String::from("segment1.ts"),
    };
    media_playlist.segments.push(media_segment1);

    let media_segment2 = MediaSegment {
        duration: Duration::new(12, 10),
        url: String::from("segment2.ts"),
    };
    media_playlist.segments.push(media_segment2);

    let media_segment3 = MediaSegment {
        duration: Duration::new(15, 10),
        url: String::from("segment3.ts"),
    };
    media_playlist.segments.push(media_segment3);

    media_playlist.version = 4;
    media_playlist.ended = true;
    media_playlist.target_duration = Duration::new(20, 0);

    media_playlist.discontinuity = Vec::new();
    //media_playlist.discontinuity.discontinuity_duration = Duration::new(0,0);

    let discontinuity_segment1 = DiscontinuitySegment {
        discontinuity_segments: vec![media_playlist.segments[0].clone()],  // creating a new vector containing a single 'MeidaSegment' struct
        discontinuity_duration: media_playlist.segments[0].duration.clone(),
    };
    media_playlist.discontinuity.push(discontinuity_segment1);

    let discontinuity_segment2 = DiscontinuitySegment {
        discontinuity_segments: vec![media_playlist.segments[1].clone()],  // creating a new vector containing a single 'MeidaSegment' struct
        discontinuity_duration: media_playlist.segments[1].duration.clone() + media_playlist.discontinuity[0].discontinuity_duration,
    };
    media_playlist.discontinuity.push(discontinuity_segment2);

    let media_segment4 = media_playlist.segments[1].clone();
    let sum_discontinuity_duration = media_playlist.segments[1].duration.as_millis()
        + media_playlist.discontinuity[0].discontinuity_duration.as_millis();
    media_playlist.discontinuity[0]
        .discontinuity_segments
        .push(media_segment4);
    media_playlist.discontinuity[0].discontinuity_duration =
        Duration::from_millis(sum_discontinuity_duration as u64);

    println!("{:?}", media_playlist.discontinuity);
}