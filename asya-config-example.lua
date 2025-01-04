local config = {
  net = {
    http_port = 3001,
  },

  logging = {
    place = false,   -- Loggin module. In log-file enable always.
    level = "debug", -- Logging level: "error", "warn", "info", "debug", "trace"
    folder = "logs", -- Folder for logs.
  },

  -- Configuration for u'r plugins.
  plugins = {
    config = {
      asya_telegram = {
        allowed_users = {
          "your_name_without_@",
        }
      }
    }
  }
}

return config
