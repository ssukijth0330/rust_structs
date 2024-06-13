use core::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct MediaPlaylist {
    ended: bool,

    // [ MediaSegment, MediaSegment, MediaSegment, MediaSegment...]
    // [ [Duration, string], [Duration, string], [Duration, string],...]
    segments: Vec<MediaSegment>,

    /// Duration that no media segment can exceed. See
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.3.1>.
    /// This is the value from the #EXT-X-TARGETDURATION tag.
    ///  secs: u64,
    /// nanos: Nanoseconds
    /// Duration:  [secs, nanos]
    target_duration: Duration,

    /// Version of playlist for compatibility. See
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.1.2>.
    version: u64,

    // The video segment between the discontinuity tag
    // [ [[Duration, string], [Duration, string], [Duration, string]...],
    //   [[Duration, string], [Duration, string], [Duration, string],...],
    //   [[Duration, string], [Duration, string], [Duration, string],...]
    //  ]
    discontinuity: Vec<DiscontinuitySegment>,
}

/// A media segment contains information to actually load the presentation. See [the
/// specification][spec] for more details.
///
/// [spec]: https://datatracker.ietf.org/doc/html/rfc8216#section-3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaSegment {
    /// From the #EXTINF tag. See <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.2.1>.
    ///  secs: u64,
    /// nanos: Nanoseconds
    /// Duration:  [secs, nanos]
    duration: Duration,

    /// Relative URL of media segment. See
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.3.2> and
    /// <https://datatracker.ietf.org/doc/html/rfc8216#section-4.1>.
    url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscontinuitySegment {
    // sum of segment durations before the EXT-X-DISCONTINUITY
    //  secs: u64,
    // nanos: Nanoseconds
    // Duration:  [secs, nanos]
    discontinuity_duration: Duration,

    // segment before the EXT-X-DISCONTINUITY
    discontinuity_segments: Vec<MediaSegment>,
}

fn main() {
    let mut media_playlist = MediaPlaylist {
        ended: false,
        segments: vec![],
        target_duration: Duration::new(0,0),
        version: 0,
        discontinuity: vec![],
    };
    // Start filling the media playlist
    let media_segment1 = MediaSegment {
        duration: Duration::new(10,1234567),
        url: String::from("segment1.ts"),
        };
    media_playlist.segments.push(media_segment1);
    let media_segment2 = MediaSegment {
        duration: Duration::new(12,10),
        url: String::from("segment2.ts"),
        };
    media_playlist.segments.push(media_segment2);
    let media_segment3 = MediaSegment {
        duration: Duration::new(15,10),
        url: String::from("segment3.ts"),
        };
    media_playlist.segments.push(media_segment3);

    media_playlist.version = 4;
    media_playlist.ended = true;
    media_playlist.target_duration = Duration::new(20,0);


    let discontinuity_segment1 = DiscontinuitySegment {
        discontinuity_duration: Duration::new(10,1234567),
        discontinuity_segments: vec![
            MediaSegment {
                duration: Duration::new(10,1234567),
                url: String::from("segment1.ts"),
            }
        ],
    };
    media_playlist.discontinuity.push(discontinuity_segment1);    

    let discontinuity_segment1 = DiscontinuitySegment {
        discontinuity_duration: Duration::new(22,20),
        discontinuity_segments: vec![
            MediaSegment {
                duration: Duration::new(15,10),
                url: String::from("segment3.ts"),
            },
        ],
    };
    media_playlist.discontinuity.push(discontinuity_segment1);

    let media_segment4 = media_playlist.segments[1].clone();
    let sum_discontinutity_duration = media_playlist.segments[1].duration.as_millis() + media_playlist.discontinuity[0].discontinuity_duration.as_millis();
    media_playlist.discontinuity[0].discontinuity_segments.push(media_segment4);
    media_playlist.discontinuity[0].discontinuity_duration = Duration::from_millis(sum_discontinutity_duration as u64);
    
    //print the media playlist
    println!("{:?}", media_playlist.discontinuity);
}
