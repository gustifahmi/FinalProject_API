<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Booking Ids</name>
   <tag></tag>
   <elementGuidId>81e0752a-6c3e-486c-b2fb-49a1791d3fdc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://restful-booker.herokuapp.com/booking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, '[0].bookingid', 229)
WS.verifyElementPropertyValue(response, '[1].bookingid', 264)
WS.verifyElementPropertyValue(response, '[2].bookingid', 25)
WS.verifyElementPropertyValue(response, '[3].bookingid', 262)
WS.verifyElementPropertyValue(response, '[4].bookingid', 129)
WS.verifyElementPropertyValue(response, '[5].bookingid', 304)
WS.verifyElementPropertyValue(response, '[6].bookingid', 184)
WS.verifyElementPropertyValue(response, '[7].bookingid', 351)
WS.verifyElementPropertyValue(response, '[8].bookingid', 136)
WS.verifyElementPropertyValue(response, '[9].bookingid', 256)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
