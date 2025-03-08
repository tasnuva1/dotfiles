-- mp.register_event("file-loaded", function()
--     local hasvid = mp.get_property_osd("video") ~= "no"
--     mp.commandv("script-message", "osc-visibility", (hasvid and "auto" or "always"), "no-osd")
--     -- remove the next line if you don't want to affect the osd-bar config
--     mp.commandv("set", "options/osd-bar", (hasvid and "yes" or "no"))
-- end)

local is_enabled = false

mp.add_key_binding("ctrl+o", "audio-osc", function() -- shortcut o
    is_enabled = not is_enabled
    -- mp.command("show-progress " .. (is_enabled and 31557600 or 0))

    mp.commandv("script-message", "osc-visibility", (is_enabled and "auto" or "always"), "no-osd")
    -- remove the next line if you don't want to affect the osd-bar config
    mp.commandv("set", "options/osd-bar", (is_enabled and "yes" or "no"))
end)

