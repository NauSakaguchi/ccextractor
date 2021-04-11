use crate::lib_ccx::ccx_common_platform::*;

pub struct ccx_s_options
{
    extract: i32,                                               // Extract 1st, 2nd or both fields
    no_rollup: i32,											   // Disable roll-up emulation (no duplicate output in generated file)
    noscte20: i32,
    webvtt_create_css: i32,
    cc_channel: i32,                                            // Channel we want to dump in srt mode
    buffer_input: i32,
    nofontcolor: i32,
    nohtmlescape: i32,
    notypesetting: i32,
    // struct ccx_boundary_time extraction_start, extraction_end; // Segment we actually process
    print_file_reports: i32,

    // ccx_decoder_608_settings settings_608;                     // Contains the settings for the 608 decoder.
    // ccx_decoder_dtvcc_settings settings_dtvcc;                 // Same for 708 decoder

    millis_separator: char,
    binary_concat: i32,                // Disabled by -ve or --videoedited
    use_gop_as_pts: i32,               // Use GOP instead of PTS timing (0=do as needed, 1=always, -1=never)
    fix_padding: i32,                  // Replace 0000 with 8080 in HDTV (needed for some cards)
    gui_mode_reports: i32,             // If 1, output in stderr progress updates so the GUI can grab them
    no_progress_bar: i32,              // If 1, suppress the output of the progress to stdout
    // char *sentence_cap_file,          // Extra capitalization word file
    live_stream: i32,                  /* -1 -> Not a complete file but a live stream, without timeout
                                          0 -> A regular file
                                         >0 -> Live stream with a timeout of this value in seconds */
    // char *filter_profanity_file;         // Extra profanity word file
    messages_target: i32,              // 0 = nowhere (quiet), 1=stdout, 2=stderr
    no_timestamp_map: i32,             // 1 for no timestamps for WebVTT, 0 for the timestamp header
    /** Levenshtein's parameters, for string comparison **/
    dolevdist: i32,					  // 0 => don't attempt to correct typos with this algorithm
    levdistmincnt: i32, levdistmaxpct: i32, // Means 2 fails or less is "the same", 10% or less is also "the same"
    investigate_packets: i32,          // Look for captions in all packets when everything else fails
    fullbin: i32,                      // Disable pruning of padding cc blocks
    nosync: i32,                       // Disable syncing
    hauppauge_mode: u32,      // If 1, use PID=1003, process specially and so on
    wtvconvertfix: i32,                // Fix broken Windows 7 conversion
    wtvmpeg2: i32,
    auto_myth: i32,                    // Use myth-tv mpeg code? 0=no, 1=yes, 2=auto
    /** MP4 related stuff **/
    mp4vidtrack: u32,             // Process the video track even if a CC dedicated track exists.
    extract_chapters: i32,		  // If 1, extracts chapters (if present), from MP4 files.
    /** General settings **/
    usepicorder: i32,                  // Force the use of pic_order_cnt_lsb in AVC/H.264 data streams
    xmltv: i32,                        // 1 = full output. 2 = live output. 3 = both
    xmltvliveinterval: i32,            // interval in seconds between writing xmltv output files in live mode
    xmltvoutputinterval: i32,          // interval in seconds between writing xmltv full file output
    xmltvonlycurrent: i32,             // 0 off 1 on
    keep_output_closed: i32,
    force_flush: i32,                  // Force flush on content write
    append_mode: i32,                  // Append mode for output files
    ucla: i32,                         // 1 if UCLA used, 0 if not
    tickertext: i32,                   // 1 if ticker text style burned in subs, 0 if not
    hardsubx: i32,                     // 1 if burned-in subtitles to be extracted
    // char *dvblang;                    // The name of the language stream for DVB
    // const char *ocrlang;              // The name of the .traineddata file to be loaded with tesseract
    ocr_oem: i32,                      // The Tesseract OEM mode, could be 0 (default), 1 or 2
    ocr_quantmode: i32,				  // How to quantize the bitmap before passing to to tesseract (0=no quantization at all, 1=CCExtractor's internal)
    // char *mkvlang;                    // The name of the language stream for MKV
    analyze_video_stream: i32,         // If 1, the video stream will be processed even if we're using a different one for subtitles.

    /**HardsubX related stuff**/
    hardsubx_ocr_mode: i32,
    hardsubx_subcolor: i32,
    hardsubx_min_sub_duration: f64,
    hardsubx_detect_italics: i32,
    hardsubx_conf_thresh: f64,
    hardsubx_hue: f64,
    hardsubx_lum_thresh: f64,

    // ccx_encoders_transcript_format transcript_settings; // Keeps the settings for generating transcript output files.
    // enum ccx_output_date_format date_format;
    send_to_srv: u32,
    // enum ccx_output_format write_format;                // 0=Raw, 1=srt, 2=SMI
    write_format_rewritten: i32,
    use_ass_instead_of_ssa: i32,
    use_webvtt_styling: i32,
    debug_mask: LLONG,                                   // dbg_print will use this mask to print or ignore different types
    debug_mask_on_debug: LLONG,                          // If we're using temp_debug to enable/disable debug "live", this is the mask when temp_debug=1
    /** Networking **/
    // char *udpsrc;
    // char *udpaddr;
    udpport: u32,                                   // Non-zero => Listen for UDP packets on this port, no files.
    // char *tcpport;
    // char *tcp_password;
    // char *tcp_desc;
    // char *srv_addr;
    // char *srv_port;
    noautotimeref: i32,                                  // Do NOT set time automatically?
    // enum ccx_datasource input_source;                   // Files, stdin or network

    // char *output_filename;

    // char **inputfile;                                   // List of files to process
    num_input_files: i32,                                // How many?
    // struct demuxer_cfg demux_cfg;
    // struct encoder_cfg enc_cfg;
    subs_delay: LLONG,                                   // ms to delay (or advance) subs
    cc_to_stdout: i32,                                   // If this is set to 1, the stdout will be flushed when data was written to the screen during a process_608 call.
    pes_header_to_stdout: i32,                           // If this is set to 1, the PES Header will be printed to console (debugging purposes)
    ignore_pts_jumps: i32,                               // If 1, the program will ignore PTS jumps. Sometimes this parameter is required for DVB subs with > 30s pause time
    multiprogram: i32,
    out_interval: i32,
    segment_on_key_frames_only: i32,
// #ifdef WITH_LIBCURL
//     char *curlposturl;
// #endif


// #ifdef ENABLE_SHARING
//     /**CC sharing**/
//     sharing_enabled: i32,
//     char *sharing_url;
//     /**Translating**/
//     translate_enabled: i32,
//     char *translate_langs;
//     char *translate_key;
// #endif
}

impl ccx_s_options
{
    pub fn new() -> Self
    {
        let ccx_options:ccx_s_options = Self
        {
            extract: 0,
            no_rollup: 0,
            noscte20: 0,
            webvtt_create_css: 0,
            cc_channel: 0,
            buffer_input: 0,
            nofontcolor: 0,
            nohtmlescape: 0,
            notypesetting: 0,
            print_file_reports: 0,
            millis_separator: '　',
            binary_concat: 0,
            use_gop_as_pts: 0,
            fix_padding: 0,
            gui_mode_reports: 0,
            no_progress_bar: 0,
            live_stream: 0,
            messages_target: 0,
            no_timestamp_map: 0,
            dolevdist: 0,
            levdistmincnt: 0,
            levdistmaxpct: 0,
            investigate_packets: 0,
            fullbin: 0,
            nosync: 0,
            hauppauge_mode: 0,
            wtvconvertfix: 0,
            wtvmpeg2: 0,
            auto_myth: 0,
            mp4vidtrack: 0,
            extract_chapters: 0,
            usepicorder: 0,
            xmltv: 0,
            xmltvliveinterval: 0,
            xmltvoutputinterval: 0,
            xmltvonlycurrent: 0,
            keep_output_closed: 0,
            force_flush: 0,
            append_mode: 0,
            ucla: 0,
            tickertext: 0,
            hardsubx: 0,
            ocr_oem: 0,
            ocr_quantmode: 0,
            analyze_video_stream: 0,
            hardsubx_ocr_mode: 0,
            hardsubx_subcolor: 0,
            hardsubx_min_sub_duration: 0.0,
            hardsubx_detect_italics: 0,
            hardsubx_conf_thresh: 0.0,
            hardsubx_hue: 0.0,
            hardsubx_lum_thresh: 0.0,
            send_to_srv: 0,
            write_format_rewritten: 0,
            use_ass_instead_of_ssa: 0,
            use_webvtt_styling: 0,
            debug_mask: 0,
            debug_mask_on_debug: 0,
            udpport: 0,
            noautotimeref: 0,
            num_input_files: 0,
            subs_delay: 0,
            cc_to_stdout: 0,
            pes_header_to_stdout: 0,
            ignore_pts_jumps: 0,
            multiprogram: 0,
            out_interval: 0,
            segment_on_key_frames_only: 0
        };
        ccx_options
    }

}