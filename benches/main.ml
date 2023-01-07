
let rec two_pow_n(x) =
   if x == 0 then 1
   else two_pow_n(x-1) + two_pow_n(x-1);;

for x = 0 to 1000 do
   two_pow_n(20) |> ignore
done;;
