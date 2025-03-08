local is_enabled = false

mp.add_key_binding("c", "osc-cycle-chapter", function() -- shortcut c
    is_enabled = not is_enabled
    mp.command("script-message osc-chapterlist " .. (is_enabled and 31557600 or 0))
end)

-- ///////////////////////////////////////////
-- F7 script-message osc-chapterlist 2 ; show-text "${osd-ass-cc/0}{\\an9}${osd-ass-cc/1}${chapter}"

-- local is_enabled = false

-- mp.add_key_binding("o", "osc-cycle-progress", function() -- shortcut c
--     is_enabled = not is_enabled
--     mp.command("show-progress " .. (is_enabled and 31557600 or 0))
-- end)


-- show-progress
-- c script-binding osc-cycle-chapter
-- o script-binding osc-cycle-progress
-- # For chapter list script
-- # c script-message osc-chapterlist


-- local mp = require("mp")

-- local show_progress = function()
-- -- mp.commandv("show-progress")
-- -- mp.commandv("osc-chapterlist")
-- 	mp.commandv("show-text")
-- end

-- -- c script-message osc-chapterlist

-- mp.register_event("file-loaded", show_progress)
-- mp.add_periodic_timer(1, show_progress)

-- //////////////////////////////////////////////////////////////////////

-- local function showChapterTitle()
-- 	local chapterTitle = mp.get_property_osd("chapter-metadata/by-key/title")
-- 	mp.osd_message(chapterTitle, 10)
-- end

-- mp.observe_property("chapter", nil, showChapterTitle

-- //////////////////////////////////////////////////////////////////////
-- function on_pause(name, value)
--     if value == true then
-- 	local chaptertitle = mp.get_property_osd("chapter")
-- 		mp.osd_message(chaptertitle,4)
--     end
-- end
-- mp.observe_property("pause", "bool", on_pause)

-- //////////////////////////////////////////////////////////////////////
-- read_options(options, 'osc_show_hide')

-- local is_hidden = true

-- local function osc_show_hide()
--   is_hidden = not is_hidden
--   mp.commandv('script-message', 'osc-visibility', (is_hidden and options.hidden_mode or 'always'), 'no-osd')
--   local visibility = require 'mp.utils'.shared_script_property_get('osc-visibility')
--   mp.commandv('script-message', 'osc-visibility', ((visibility == 'auto' or visibility == 'never') and 'always' or options.hidden_mode), 'no-osd')
-- end

-- mp.add_key_binding('/', 'osc-show-hide', osc_show_hide)
-- Footer

