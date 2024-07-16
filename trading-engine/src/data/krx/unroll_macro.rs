#[macro_export]
macro_rules! parse_unroll_unchecked_price_with_buffer {
    ($max_level:expr, $quote_start_index:expr, $offset:expr, $payload:expr, $data_buffer:expr, $converter:expr, $pr_ln:expr, $qn_ln:expr, $or_ln:expr) => {
        unsafe {
            if $max_level >= 1 {
                let st_idx_marker = $quote_start_index;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[0].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[0].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[0].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[0].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[0].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[0].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 2 {
                let st_idx_marker = $quote_start_index + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[1].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[1].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[1].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[1].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[1].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[1].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }
            
            if $max_level >= 3 {
                let st_idx_marker = $quote_start_index + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[2].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[2].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[2].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[2].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[2].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[2].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 4 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[3].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[3].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[3].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[3].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[3].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[3].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 5 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[4].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[4].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[4].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[4].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[4].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[4].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 6 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[5].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[5].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[5].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[5].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[5].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[5].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 7 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[6].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[6].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[6].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[6].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[6].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[6].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 8 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[7].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[7].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[7].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[7].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[7].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[7].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 9 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[8].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[8].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[8].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[8].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[8].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[8].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 10 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[9].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[9].book_price =
                    $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[9].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[9].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[9].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[9].order_count = $converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }
        }
    };
}

#[macro_export]
macro_rules! parse_unroll_unchecked_price {
    (
        $max_level:expr, 
        $quote_start_index:expr, 
        $offset:expr, 
        $payload:expr, 
        $ask_quote_data:expr, 
        $bid_quote_data:expr,
        $converter:expr, 
        $pr_ln:expr, 
        $qn_ln:expr, 
        $or_ln:expr
    ) => {
        unsafe {
            if $max_level >= 1 {
                let st_idx_marker = $quote_start_index;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[0].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[0].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[0].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[0].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[0].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[0].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 2 {
                let st_idx_marker = $quote_start_index + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[1].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[1].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[1].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[1].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[1].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[1].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 3 {
                let st_idx_marker = $quote_start_index + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[2].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[2].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[2].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[2].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[2].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[2].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 4 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[3].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[3].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[3].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[3].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[3].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[3].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 5 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[4].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[4].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[4].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[4].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[4].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[4].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 6 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[5].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[5].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[5].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[5].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[5].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[5].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 7 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[6].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[6].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[6].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[6].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[6].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[6].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 8 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[7].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[7].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[7].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[7].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[7].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[7].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 9 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[8].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[8].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[8].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[8].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[8].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[8].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }

            if $max_level >= 10 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[9].book_price = $converter.to_book_price_unchecked(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[9].book_price = $converter.to_book_price_unchecked(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[9].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[9].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[9].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]);
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[9].order_count = $converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]);
            }
        }
    }
}

#[macro_export]
macro_rules! parse_unroll_with_buffer {
    ($max_level:expr, $quote_start_index:expr, $offset:expr, $payload:expr, $data_buffer:expr, $converter:expr, $pr_ln:expr, $qn_ln:expr, $or_ln:expr) => {
        unsafe {
            if $max_level >= 1 {
                let st_idx_marker = $quote_start_index;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[0].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[0].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[0].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[0].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[0].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[0].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 2 {
                let st_idx_marker = $quote_start_index + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[1].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[1].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[1].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[1].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[1].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[1].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }
            
            if $max_level >= 3 {
                let st_idx_marker = $quote_start_index + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[2].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[2].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[2].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[2].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[2].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[2].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 4 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[3].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[3].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[3].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[3].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[3].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[3].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 5 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[4].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[4].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[4].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[4].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[4].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[4].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 6 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[5].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[5].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[5].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[5].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[5].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[5].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 7 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[6].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[6].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[6].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[6].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[6].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[6].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 8 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[7].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[7].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[7].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[7].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[7].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[7].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 9 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[8].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[8].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[8].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[8].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[8].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[8].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 10 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $data_buffer.ask_quote_data[9].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $data_buffer.bid_quote_data[9].book_price =
                    $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $data_buffer.ask_quote_data[9].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $data_buffer.bid_quote_data[9].book_quantity = $converter
                    .to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $data_buffer.ask_quote_data[9].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $data_buffer.bid_quote_data[9].order_count = Some($converter
                    .to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }
        }
    };
}

#[macro_export]
macro_rules! parse_unroll {
    (
        $max_level:expr, 
        $quote_start_index:expr, 
        $offset:expr, 
        $payload:expr, 
        $ask_quote_data:expr, 
        $bid_quote_data:expr,
        $converter:expr, 
        $pr_ln:expr, 
        $qn_ln:expr, 
        $or_ln:expr
    ) => {
        unsafe {
            if $max_level >= 1 {
                let st_idx_marker = $quote_start_index;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[0].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[0].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[0].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[0].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[0].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[0].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 2 {
                let st_idx_marker = $quote_start_index + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[1].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[1].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[1].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[1].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[1].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[1].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 3 {
                let st_idx_marker = $quote_start_index + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[2].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[2].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[2].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[2].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[2].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[2].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 4 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[3].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[3].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[3].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[3].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[3].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[3].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 5 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[4].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[4].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[4].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[4].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[4].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[4].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 6 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[5].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[5].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[5].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[5].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[5].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[5].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 7 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[6].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[6].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[6].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[6].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[6].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[6].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 8 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[7].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[7].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[7].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[7].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[7].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[7].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 9 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[8].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[8].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[8].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[8].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[8].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[8].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }

            if $max_level >= 10 {
                let st_idx_marker = $quote_start_index + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset + $offset;
                let payload_clipped = &$payload[st_idx_marker..st_idx_marker + $offset];

                $ask_quote_data[9].book_price = $converter.to_book_price(&payload_clipped[0..$pr_ln]);
                let idx_marker1 = $pr_ln + $pr_ln;
                $bid_quote_data[9].book_price = $converter.to_book_price(&payload_clipped[$pr_ln..idx_marker1]);

                $ask_quote_data[9].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker1..idx_marker1 + $qn_ln]);
                let idx_marker2 = idx_marker1 + $qn_ln;
                $bid_quote_data[9].book_quantity = $converter.to_book_quantity_unchecked(&payload_clipped[idx_marker2..idx_marker2 + $qn_ln]);

                let idx_marker3 = idx_marker2 + $qn_ln;
                $ask_quote_data[9].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker3..idx_marker3 + $or_ln]));
                let idx_marker4 = idx_marker3 + $or_ln;

                $bid_quote_data[9].order_count = Some($converter.to_order_count_unchecked(&payload_clipped[idx_marker4..idx_marker4 + $or_ln]));
            }
        }
    }
}