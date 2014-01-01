def fib(i)
  if i == 0 or i == 1
    i
  else
    fib(i-2) + fib(i-1)
  end
end

if $0 == __FILE__
  (0..20).each do |i|
    puts "fib(%d)=%d" % [i, fib(i)]
  end
end
