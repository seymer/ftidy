--- Yazi plugin for ftidy integration
--- Provides dedup and rename commands for selected files

local function selected_or_cwd()
	local h = cx.active.current.hovered
	if h then
		return tostring(h.url:parent() or h.url)
	end
	return ""
end

local function selected_files()
	local files = {}
	for _, f in pairs(cx.active.selected) do
		files[#files + 1] = tostring(f)
	end
	if #files == 0 then
		local h = cx.active.current.hovered
		if h then
			files[#files + 1] = tostring(h.url)
		end
	end
	return files
end

return {
	entry = function(_, job)
		local action = job.args[1] or "dedup"

		if action == "dedup" then
			local dir = selected_or_cwd()
			ya.manager_emit("shell", {
				"ftidy dedup " .. ya.quote(dir) .. " --delete",
				block = true,
				confirm = true,
			})
		elseif action == "rename" then
			local files = selected_files()
			if #files == 0 then
				return ya.notify { title = "ftidy", content = "No files selected", level = "warn", timeout = 3 }
			end
			local args = ""
			for _, f in ipairs(files) do
				args = args .. " " .. ya.quote(f)
			end
			ya.manager_emit("shell", {
				'read -p "Pattern ({n},{name},{ext}): " p && ftidy rename' .. args .. ' -p "$p"',
				block = true,
				confirm = true,
			})
		else
			ya.notify { title = "ftidy", content = "Unknown action: " .. action, level = "error", timeout = 3 }
		end
	end,
}
