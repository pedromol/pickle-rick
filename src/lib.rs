use neon::prelude::*;
use neon::event::Channel;

use std::fs::File;
use std::io::Read;
use std::error::Error;

use serde_pickle as pickle;

fn unpickle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // The types `String`, `Root<JsFunction>`, and `Channel` can all be
    // sent across threads.
    let filename = cx.argument::<JsString>(0)?.value(&mut cx);
    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let channel = cx.channel();

    // Spawn a background thread to complete the execution. The background
    // execution will _not_ block the JavaScript event loop.
    std::thread::spawn(move || {
        // Do the heavy lifting inside the background thread.
        parse_unpickle(filename, callback, channel);
    });

    Ok(cx.undefined())
}

fn parse_unpickle(filename: String, callback: Root<JsFunction>, channel: Channel) {
    // Send a closure as a task to be executed by the JavaScript event
    // loop. This _will_ block the event loop while executing.
    channel.send(move |mut cx| {
        let callback = callback.into_inner(&mut cx);
        let this = cx.undefined();

        let args = match serde_unpickle(filename) {
            Ok(parsed) => {
                let result = cx.string(parsed);
                vec![
                    cx.null().upcast::<JsValue>(),
                    result.upcast::<JsValue>(),
                ]
            }
            Err(err) => {
                let err = cx.string(err.to_string());
                vec![
                    err.upcast::<JsValue>(),
                ]
            }
        };

        callback.call(&mut cx, this, args)?;

        Ok(())
    });
}

fn serde_unpickle(filename: String) -> Result<String, Box<dyn Error>> {
    let reader: Box<dyn Read> = Box::new(File::open(filename)?);
    let decoded: pickle::Value = pickle::value_from_reader(reader, Default::default())?;
    Ok(decoded.to_string())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("unpickle", unpickle)?;
    Ok(())
}
