local get_cwd = ya.sync(function()
	return tostring(cx.active.current.cwd)
end)

local get_selected = ya.sync(function()
	local files = {}
	for _, u in pairs(cx.active.selected) do
		files[#files + 1] = tostring(u)
	end
	if #files == 0 then
		local h = cx.active.current.hovered
		if h and not h.cha.is_dir then
			files[#files + 1] = tostring(h.url)
		end
	end
	return files
end)

return {
	entry = function(_, job)
		local action = job.args[1] or "dedup"

		if action == "dedup" then
			local cwd = get_cwd()

			-- Scan for duplicates
			local output = Command("ftidy"):arg("dedup"):arg(cwd):arg("--keep"):arg("oldest")
				:stdout(Command.PIPED):stderr(Command.PIPED):output()

			if not output or not output.status.success then
				ya.notify({ title = "ftidy", content = output and output.stderr or "Failed to run", level = "error", timeout = 3 })
				return
			end

			if output.stdout:match("^No duplicates") then
				ya.notify({ title = "ftidy", content = "No duplicates found", level = "info", timeout = 3 })
				return
			end

			-- Ask to delete
			local yes = ya.confirm({
				pos = { "center", w = 60, h = 15 },
				title = "ftidy dedup (keep=oldest)",
				body = output.stdout .. "\nDelete duplicates?",
			})

			if yes then
				local del = Command("ftidy"):arg("dedup"):arg(cwd):arg("--keep"):arg("oldest"):arg("--delete")
					:stdin(Command.PIPED):stdout(Command.PIPED):stderr(Command.PIPED):spawn()
				del:write_all("y\n")
				del:flush()
				local result = del:wait_with_output()
				if result.status.success then
					ya.notify({ title = "ftidy", content = "Duplicates deleted", level = "info", timeout = 3 })
				else
					ya.notify({ title = "ftidy", content = result.stderr, level = "error", timeout = 5 })
				end
			end

		elseif action == "rename" then
			local files = get_selected()
			if #files == 0 then
				ya.notify({ title = "ftidy", content = "No files selected", level = "warn", timeout = 3 })
				return
			end

			local pattern, event = ya.input({
				title = "Rename pattern ({n},{name},{ext}):",
				pos = { "center", w = 50 },
			})
			if event ~= 1 or not pattern or pattern == "" then return end

			local cmd = Command("ftidy"):arg("rename")
			for _, f in ipairs(files) do
				cmd = cmd:arg(f)
			end
			local output = cmd:arg("-p"):arg(pattern)
				:stdout(Command.PIPED):stderr(Command.PIPED):output()

			if output.status.success then
				ya.notify({ title = "ftidy", content = output.stdout, level = "info", timeout = 5 })
			else
				ya.notify({ title = "ftidy", content = output.stderr, level = "error", timeout = 5 })
			end
		end
	end,
}
