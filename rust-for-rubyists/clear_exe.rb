#!/home/itang/.rvm/rubies/ruby-2.0.0-p353/bin/ruby

Dir.foreach(".") do |x| 
  # puts "Got #{x}"
  if !(x =~ /\.rs/) and !(x =~ /\.rb/)
    puts x
    unless File.directory?(x)
      File.delete(x)
    end
  end
end