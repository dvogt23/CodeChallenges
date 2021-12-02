class LogLineParser
  DELIMITER = ':'

  def initialize(line)
    @line = line
  end

  def message
    @line.slice(colon + 2, @line.size).strip
  end

  def log_level
    @line.slice(1, colon - 2).downcase
  end

  def reformat
    "#{message} (#{log_level})"
  end

  private

  def colon
    @line.index(DELIMITER)
  end
end
