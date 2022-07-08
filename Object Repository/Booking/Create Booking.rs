<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Booking</name>
   <tag></tag>
   <elementGuidId>f428b261-b9a8-4e96-bece-8288b6f64c16</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;firstname\&quot; : \&quot;Boris\&quot;,\n  \&quot;lastname\&quot; : \&quot;Johnson\&quot;,\n  \&quot;totalprice\&quot; : 150000,\n  \&quot;depositpaid\&quot; : true,\n  \&quot;bookingdates\&quot; : {\n    \&quot;checkin\&quot; : \&quot;2022-07-07\&quot;,\n    \&quot;checkout\&quot; : \&quot;2022-07-08\&quot;\n  },\n  \&quot;additionalneeds\&quot; : \&quot;Lunch\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0801f4ab-c087-4f16-bc64-7e1100d8b40a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>09226576-e560-42c5-ab4d-a0a639bfd081</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
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

WS.verifyElementPropertyValue(response, 'booking.firstname', &quot;Boris&quot;)
WS.verifyElementPropertyValue(response, 'booking.lastname', &quot;Johnson&quot;)
WS.verifyElementPropertyValue(response, 'booking.totalprice', 150000)
WS.verifyElementPropertyValue(response, 'booking.depositpaid', true)
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkin', &quot;2022-07-07&quot;)
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkout', &quot;2022-07-08&quot;)
WS.verifyElementPropertyValue(response, 'booking.additionalneeds', &quot;Lunch&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
