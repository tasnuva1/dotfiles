local is_enabled = false

mp.add_key_binding("f7", "osc-left-time-chapter", function() -- shortcut c
    is_enabled = not is_enabled
    mp.command("script-message osc-chapterlist 2 ; show-text '${osd-ass-cc/0}{\\an9}${osd-ass-cc/1}${chapter}'" .. (is_enabled and 31557600 or 0))
end)

