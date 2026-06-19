#!/bin/bash
xvfb-run --server-args="-screen 0 1280x1024x24" bash -c '
  cargo run -p image-viewer &
  APP_PID=$!
  sleep 4
  import -window root /home/kabir/.gemini/antigravity/brain/ac319c7f-ec88-4151-a098-fcd2e94d7f09/screenshot_test.png
  kill $APP_PID
'
