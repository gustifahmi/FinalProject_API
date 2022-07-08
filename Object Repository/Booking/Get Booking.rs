<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Booking</name>
   <tag></tag>
   <elementGuidId>37f8d094-1e98-44ce-bce1-ddde2c0c0085</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>12861a36-2b52-4d47-b5ec-865457097a89</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://restful-booker.herokuapp.com/booking/229</restUrl>
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

WS.verifyElementPropertyValue(response, 'firstname', &quot;Michael&quot;)
WS.verifyElementPropertyValue(response, 'lastname', &quot;%date:~4,2%%date:~7,2%%date:~10,4%_%time:~0,2%%time:~3,2%%time:~6,2%&quot;)
WS.verifyElementPropertyValue(response, 'totalprice', 111)
WS.verifyElementPropertyValue(response, 'depositpaid', true)
WS.verifyElementPropertyValue(response, 'bookingdates.checkin', &quot;2019-05-16&quot;)
WS.verifyElementPropertyValue(response, 'bookingdates.checkout', &quot;2019-05-16&quot;)
WS.verifyElementPropertyValue(response, 'additionalneeds', &quot;Breakfast&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
