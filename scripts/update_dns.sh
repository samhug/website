#!/bin/bash

HOSTS="muelh.ug sa.muelh.ug"
HE_KEY="<secret>"

current4=`curl -4 -s http://checkip.dns.he.net | grep -o -E '([0-9]{1,3}\.){3}[0-9]{1,3}'`
current6=`curl -6 -s http://checkip.dns.he.net | grep -o -E '([A-Fa-f0-9]{1,4}:){7}[A-Fa-f0-9]{1,4}'`

echo "IPv4: $current4"
echo "IPv6: $current6"

for h in $HOSTS; do
	dns4=`host -4 -t A $h | grep -o -E '([0-9]{1,3}\.){3}[0-9]{1,3}'`
	dns6=`host -6 -t A $h | grep -o -E '([0-9]{1,3}\.){3}[0-9]{1,3}'`

	HE_URL="http://$h:$HE_KEY@dyn.dns.he.net/nic/update?hostname=$h"

	if [ "$current4" != "$dns4" ]; then
		r4=$(curl -4 $HE_URL)
		if ! [[ "$r4" =~ "good" || "$r4" =~ "nochg" ]]; then
			logger -p local0.err -t HE-DDNS  "DNS update failed for IPv4 $h: $r4"
		fi
	fi

	if [ "$current6" != "$dns6" ]; then
		r6=$(curl -6 $HE_URL)
		if ! [[ "$r6" =~ "good" || "$r6" =~ "nochg" ]]; then
			logger -p local0.err -t HE-DDNS  "DNS update failed for IPv6 $h: $r6"
		fi
	fi

done
